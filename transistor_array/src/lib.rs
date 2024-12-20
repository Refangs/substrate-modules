// begin-code-snippet imports
use serde::{Deserialize, Serialize};
use sky130pdk::mos::{Pfet01v8};
use sky130pdk::Sky130Pdk;
use substrate::block::Block;
use substrate::io::{Input, Output, Signal};
use substrate::io::Io;
use substrate::io::schematic::HardwareType;
use substrate::schematic::{CellBuilder, ExportsNestedData, Schematic};
use substrate::io::Array;
// end-code-snippet imports

pub mod tb;

// begin-code-snippet transistor-array-io
#[derive(Io, Clone, Debug)]
pub struct TransistorArrayIo {
    pub vg: Input<Array<Signal>>,
    pub vin: Input<Signal>,
    pub vout: Output<Signal>,
}

impl Default for TransistorArrayIo {
   fn default() -> Self {
       Self {
           vg: Input(Array::new(1, Signal::new())),
           vin: Input(Signal::new()),
           vout: Output(Signal::new()),
       }
   }
}

impl TransistorArrayIo {
    pub fn new(num_transistors: usize) -> Self {
        Self {
            vg: Input(Array::new(num_transistors, Signal::new())),
            vin: Input(Signal::new()),
            vout: Output(Signal::new()),
        }
    }
}
// end-code-snippet transistor-array-io

use substrate::arcstr;
use substrate::arcstr::ArcStr;
impl Block for TransistorArray {
    type Io = TransistorArrayIo;

    fn id() -> ArcStr {
        arcstr::literal!("transistorarray")
    }

    fn name(&self) -> ArcStr {
        arcstr::format!("transistorarray_pw{}_lch{}_num{}", self.pw, self.lch, self.num)
    }
    
    fn io(&self) -> Self::Io {
        TransistorArrayIo::new(self.num as usize)
    }
}

// begin-code-snippet transistor-array-struct
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
//#[substrate(io = "TransistorArrayIo")]
pub struct TransistorArray {
    /// PMOS width.
    pub pw: i64,
    /// Channel length.
    pub lch: i64,
    /// Number of transistors.
    pub num: usize,
}
// end-code-snippet transistor-array-struct

// begin-code-snippet transistor-array-schematic
impl ExportsNestedData for TransistorArray {
    type NestedData = ();
}

impl Schematic<Sky130Pdk> for TransistorArray {
    fn schematic(
        &self,
        io: &<<Self as Block>::Io as HardwareType>::Bundle,
        cell: &mut CellBuilder<Sky130Pdk>,
    ) -> substrate::error::Result<Self::NestedData> {
        for n in 0..self.num {
            let curr_pmos = cell.instantiate(Pfet01v8::new((self.pw, self.lch)));
            cell.connect(io.vg[n as usize], curr_pmos.io().g);
            cell.connect(io.vin, curr_pmos.io().s);
            cell.connect(io.vout, curr_pmos.io().d);
            cell.connect(io.vin, curr_pmos.io().b);
        }

        Ok(())
    }
}
// end-code-snippet transistor-array-schematic
