use mini_agent_core::event::{Event, Metric};
use sysinfo::{Pid, Process, SystemExt};
use tokio::sync::mpsc;

use super::prelude::SourceConfig;
use crate::prelude::Component;

#[derive(Debug, serde::Deserialize)]
pub struct SysinfoConfig {
    pub interval: f64,
}

impl SourceConfig for SysinfoConfig {
    fn build(self, output: mpsc::Sender<Event>) -> super::Source {
        super::Source::Sysinfo(Sysinfo {
            interval: tokio::time::interval(tokio::time::Duration::from_secs_f64(self.interval)),
            system: sysinfo::System::new_all(),
            output,
        })
    }
}

fn combine_cmd(input: &[String]) -> String {
    input
        .iter()
        .map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(" ")
}

pub struct Sysinfo {
    interval: tokio::time::Interval,
    system: sysinfo::System,
    output: mpsc::Sender<Event>,
}

impl Sysinfo {
    async fn push(&self, metric: Metric) {
        let event = metric
            .with_optional_tag("host", self.system.host_name())
            .into();
        if let Err(err) = self.output.send(event).await {
            eprintln!("unable to send event: {err:?}");
        }
    }

    async fn handle_components(&self) {
        use sysinfo::ComponentExt;

        for comp in self.system.components() {
            let name = comp.label().to_owned();
            self.push(
                Metric::now("system.component.temperature", comp.temperature() as f64)
                    .with_tag("name", name.clone()),
            )
            .await;
        }
    }

    async fn handle_cpu(&self) {
        use sysinfo::CpuExt;

        self.push(Metric::now("system.cpu.count", self.system.cpus().len() as f64))
            .await;
        for cpu in self.system.cpus() {
            self.push(
                Metric::now("system.cpu.usage", cpu.cpu_usage() as f64)
                    .with_tag("name", cpu.name().to_string()),
            )
            .await;
            self.push(
                Metric::now("system.cpu.frequency", cpu.frequency() as f64)
                    .with_tag("name", cpu.name().to_string()),
            )
            .await;
        }
    }

    async fn handle_memory(&self) {
        futures::join!(
            self.push(Metric::now("system.memory.total", self.system.total_memory() as f64)),
            self.push(Metric::now("system.memory.used", self.system.used_memory() as f64)),
            self.push(Metric::now("system.swap.total", self.system.total_swap() as f64)),
            self.push(Metric::now("system.swap.used", self.system.used_swap() as f64)),
        );
    }

    async fn handle_process(&self, pid: &Pid, process: &Process) {
        use sysinfo::ProcessExt;

        let tags = vec![
            ("cmd", combine_cmd(process.cmd())),
            ("exe", process.exe().to_string_lossy().to_string()),
            ("pid", format!("{pid}")),
            ("cwd", process.cwd().to_string_lossy().to_string()),
        ];

        futures::join!(
            self.push(
                Metric::now("process.memory", process.memory() as f64).with_tags(tags.clone())
            ),
            self.push(
                Metric::now("process.virtual_memory", process.virtual_memory() as f64)
                    .with_tags(tags.clone())
            ),
            self.push(
                Metric::now("process.cpu_usage", process.cpu_usage() as f64)
                    .with_tags(tags.clone())
            ),
        );
    }

    async fn handle_processes(&self) {
        for (pid, process) in self.system.processes() {
            self.handle_process(pid, process).await;
        }
    }
}

impl Component for Sysinfo {
    async fn run(mut self) {
        loop {
            self.system.refresh_all();
            futures::join!(
                self.handle_memory(),
                self.handle_processes(),
                self.handle_cpu(),
                self.handle_components(),
            );
            let _ = self.interval.tick().await;
        }
    }
}
