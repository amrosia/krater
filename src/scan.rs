use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[derive(Debug)]
pub struct Scan {
    pub url: String,
    pub modules: Vec<String>,
    pub options: Option<ScanOptions>,
    running: Arc<AtomicBool>,
}

impl Scan {
    fn new(running: Arc<AtomicBool>) -> Self {
        Self {
            url: String::new(),
            modules: Vec::new(),
            options: None,
            running,
        }
    }

    pub async fn run(input: &str, running: Arc<AtomicBool>) -> Result<(), String> {
        let mut scan = Scan::new(running);
        scan.parse_input(input)?;
        
        println!("Running scan for URL: {}", scan.url);
        
        let timeout = scan.options
            .as_ref()
            .map(|opt| opt.timeout)
            .unwrap_or(240);

        for i in 1..=5 {
            scan.modules.push(format!("whois{}", i).to_string());
        }

        for module in &scan.modules {
            if !scan.running.load(Ordering::SeqCst) {
                return Err("Scan interrupted".to_string());
            }

            tokio::select! {
                _ = async {
                    for i in 1..=5 {
                        if !scan.running.load(Ordering::SeqCst) {
                            return;
                        }
                        println!("Module {} progress: {}0%", module, i);
                        tokio::time::sleep(std::time::Duration::from_secs(timeout/5)).await;
                    }
                } => {
                    if scan.running.load(Ordering::SeqCst) {
                        println!("Module {} completed", module);
                    }
                }
            }
        }
        
        if !scan.running.load(Ordering::SeqCst) {
            return Err("Scan interrupted".to_string());
        }

        Ok(())
    }

    /*
    - Does it start with -- or -? Argument.
    - Does it have a dot, or starts with http/https? Domain
    - None of that? I don't know then!
     */
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        let args: Vec<&str> = input.split_whitespace().skip(1).collect(); // skip "scan" command
        let mut scan_options = ScanOptions::new(240, true);

        let mut i = 0;
        while i < args.len() {
            match args[i] {
                arg if arg.starts_with("--") => {
                    match arg {
                        "--quiet" => scan_options.verbose = false,
                        "--timeout" => {
                            if i + 1 < args.len() {
                                if let Ok(timeout) = args[i + 1].parse::<u64>() {
                                    scan_options.timeout = timeout;
                                    i += 1;
                                } else {
                                    return Err(format!("Invalid timeout value: {}", args[i + 1]));
                                }
                            }
                        }
                        _ => return Err(format!("Unknown option: {}", arg)),
                    }
                }
                arg if arg.starts_with("http://") || arg.starts_with("https://") || arg.contains(".") => {
                    self.url = arg.to_string();
                }
                e => {return Err(format!("Can't figure out what is '{e}' you are giving me"));},
            }
            i += 1;
        }

        if self.url.is_empty() {
            return Err("No valid URL provided".to_string());
        }
        self.options = Some(scan_options);
        Ok(())
    }
}

#[derive(Debug)]
pub struct ScanOptions {
    pub timeout: u64,
    pub verbose: bool,
}

impl ScanOptions {
    pub fn new(timeout: u64, verbose: bool) -> Self {
        Self { timeout, verbose }
    }
}

