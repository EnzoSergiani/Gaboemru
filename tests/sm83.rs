#[cfg(test)]
mod tests {
    use gaboemru::common::bus::Bus;
    use serde::Deserialize;

    use gaboemru::common::test_helpers::FlatRam;
    use gaboemru::cpu::{
        Cpu,
        registers::{Flags, Registers},
    };

    #[derive(Deserialize)]
    struct CpuState {
        pc: u16,
        sp: u16,
        a: u8,
        b: u8,
        c: u8,
        d: u8,
        e: u8,
        f: u8,
        h: u8,
        l: u8,
        ime: u8,
        ram: Vec<(u16, u8)>,
    }

    #[derive(Deserialize)]
    struct TestCase {
        name: String,
        initial: CpuState,
        #[serde(rename = "final")]
        expected: CpuState,
        // cycles: Vec<serde_json::Value>,
    }

    fn build_cpu(state: &CpuState) -> Cpu {
        let mut cpu = Cpu::new();
        let registers = Registers {
            a: state.a,
            b: state.b,
            c: state.c,
            d: state.d,
            e: state.e,
            h: state.h,
            l: state.l,
            f: Flags::from_byte(state.f),
            sp: state.sp,
            pc: state.pc,
        };
        cpu.set_state(registers, state.ime == 1);
        cpu
    }

    fn build_bus(state: &CpuState) -> FlatRam {
        let mut ram = FlatRam::default();
        for (addr, value) in &state.ram {
            ram.write(*addr, *value);
        }
        ram
    }

    #[test]
    fn singlesteptests_sm83_all_pass() {
        let dir = "tests/data/sm83/v1";
        let mut failures: Vec<String> = Vec::new();
        let mut total = 0usize;

        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }

            let content = std::fs::read_to_string(&path).unwrap();
            let cases: Vec<TestCase> = serde_json::from_str(&content).unwrap();

            for case in cases {
                total += 1;
                let mut cpu = build_cpu(&case.initial);
                let mut bus = build_bus(&case.initial);

                cpu.step(&mut bus);

                let (regs, ime) = cpu.state();
                let ok = regs.a == case.expected.a
                    && regs.b == case.expected.b
                    && regs.c == case.expected.c
                    && regs.d == case.expected.d
                    && regs.e == case.expected.e
                    && regs.h == case.expected.h
                    && regs.l == case.expected.l
                    && regs.f.to_byte() == case.expected.f
                    && regs.sp == case.expected.sp
                    && regs.pc == case.expected.pc
                    && ime == (case.expected.ime == 1)
                    && case
                        .expected
                        .ram
                        .iter()
                        .all(|(addr, val)| bus.read(*addr) == *val);

                if !ok {
                    failures.push(format!("{}: {}", path.display(), case.name));
                }
            }
        }

        assert!(
            failures.is_empty(),
            "{}/{} cas échoués:\n{}",
            failures.len(),
            total,
            failures.join("\n")
        );
    }
}
