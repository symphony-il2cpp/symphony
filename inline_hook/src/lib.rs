use std::{ffi::c_void, mem, ptr};

const A64_MAX_INSTRUCTIONS: usize = 5;
const A64_MAX_REFERENCES: usize = A64_MAX_INSTRUCTIONS * 2;

type Instruction = *mut *mut u32;

struct FixInfo {
    bp: *mut u32,
    ls: u32,
    ad: u32,
}

union InsnsInfoValue {
    insu: u64,
    ins: i64,
    insp: *mut c_void,
}

struct InsnsInfo {
    value: InsnsInfoValue,
    fmap: [FixInfo; A64_MAX_REFERENCES],
}

struct Context {
    basep: i64,
    endp: i64,
    dat: [InsnsInfo; A64_MAX_INSTRUCTIONS],
}

impl Context {
    fn is_in_fixing_range(&self, absolute_addr: i64) -> bool {
        absolute_addr >= self.basep && absolute_addr < self.endp
    }
    fn get_ref_ins_index(&self, absolute_addr: i64) -> usize {
        ((absolute_addr - self.basep) / 4) as usize
    }
    unsafe fn get_and_set_current_index(&mut self, inp: *mut u32, outp: *mut u32) -> usize {
        let current_idx = self.get_ref_ins_index(mem::transmute(inp));
        self.dat[current_idx].value.insp = outp as *mut c_void;
        current_idx
    }
    fn reset_current_ins(&mut self, idx: usize, outp: *mut u32) {
        self.dat[idx].value.insp = outp as *mut c_void;
    }
    fn insert_fix_map(&mut self, idx: usize, bp: *mut u32, ls: Option<u32>, ad: Option<u32>) {
        let ls = ls.unwrap_or(0);
        let ad = ad.unwrap_or(0xffff_ffff);

        for f in self.dat[idx].fmap.iter_mut() {
            if f.bp.is_null() {
                f.bp = bp;
                f.ls = ls;
                f.ad = ad;
                return;
            }
        }
    }
    unsafe fn process_fix_map(&mut self, idx: usize) {
        for f in self.dat[idx].fmap.iter_mut() {
            if f.bp.is_null() {
                break;
            }
            *f.bp |= (((self.dat[idx].value.ins - mem::transmute::<*mut u32, i64>(f.bp)) as i32
                >> 2)
                << f.ls as i32) as u32
                & f.ad;
            f.bp = ptr::null_mut();
        }
    }
}

unsafe fn fix_branch_imm(inpp: Instruction, outpp: Instruction, ctx: &mut Context) -> bool {
    const MBITS: u32 = 6;
    const MASK: u32 = 0xfc00_0000;
    const RMASK: u32 = 0x03ff_ffff;
    const OP_B: u32 = 0x1400_0000;
    const OP_BL: u32 = 0x9400_0000;

    let ins = **inpp;
    let opc = ins & MASK;
    match opc {
        OP_B | OP_BL => {
            let current_idx = ctx.get_and_set_current_index(*inpp, *outpp);
            let absolute_addr = mem::transmute::<*mut u32, i64>(*inpp)
                + ((ins << MBITS) as i32 >> (MBITS - 2) as i32) as i64;
            let new_pc_offset = (absolute_addr - mem::transmute::<*mut u32, i64>(*outpp)) >> 2;
            let special_fix_type = ctx.is_in_fixing_range(absolute_addr);
            if !special_fix_type && new_pc_offset.abs() as u32 >= (RMASK >> 1) {}
            let b_aligned = (mem::transmute(*outpp.add(2)) & 7) == 0;

            true
        }
        _ => false,
    }
}
