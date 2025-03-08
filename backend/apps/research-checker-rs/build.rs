use std::io::Result;

use poem_grpc_build::compile_protos;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=../../contracts/research_checker/research_checker.proto");
    compile_protos(
        &[
            "../../contracts/research_checker/research_checker.proto",
            "../../contracts/research_parser/research_parser.proto",
        ],
        &[
            "../../contracts/research_checker/",
            "../../contracts/research_parser/",
        ],
    )
}
