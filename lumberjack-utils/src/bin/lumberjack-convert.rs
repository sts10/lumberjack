use std::io::{BufReader, Read};

use clap::{App, AppSettings, Arg, ArgMatches};
use failure::{format_err, Error};
use stdinout::{Input, OrExit, Output};

use lumberjack::negra_reader::NegraTreeIter;
use lumberjack::ptb_reader::{PTBBuilder, PTBFormat, PTBTreeIter};
use lumberjack::ptb_writer::PTBFormatter;
use lumberjack::tree::Tree;

fn main() {
    let matches = parse_args();
    let in_path = matches.value_of(INPUT).map(ToOwned::to_owned);
    let input = Input::from(in_path);
    let reader = BufReader::new(input.buf_read().or_exit("Can't open input reader.", 1));
    let out_path = matches.value_of(OUTPUT).map(ToOwned::to_owned);
    let output = Output::from(out_path);
    let mut writer = output.write().or_exit("Can't open output writer.", 1);
    let in_format = matches.value_of(IN_FORMAT).unwrap_or("ptb-singleline");
    let in_format = InputFormat::try_from_str(in_format).or_exit("Can't read input format.", 1);
    let out_format = matches.value_of(OUT_FORMAT).unwrap_or("simple");
    let out_formatter =
        PTBFormatter::try_from_str(out_format).or_exit("Can't read output format.", 1);

    for tree in get_reader(in_format, reader) {
        let mut tree = tree.or_exit("Could not read tree.", 1);
        tree.projectivize();
        let tree_string = out_formatter
            .format(&tree)
            .or_exit("Can't linearize tree.", 1);
        writeln!(writer, "{}", tree_string).or_exit("Can't write to output.", 1);
    }
}

fn get_reader<'a, R>(
    in_format: InputFormat,
    input: BufReader<R>,
) -> Box<Iterator<Item = Result<Tree, Error>> + 'a>
where
    R: Read + 'a,
{
    match in_format {
        InputFormat::PTBSingleLine => Box::new(PTBTreeIter::new_with_defaults(input)),
        InputFormat::PTBMultiLine => Box::new(PTBTreeIter::new(
            input,
            PTBBuilder::default(),
            PTBFormat::MultiLine,
        )),
        InputFormat::NEGRA => Box::new(NegraTreeIter::new(input)),
    }
}

enum InputFormat {
    PTBSingleLine,
    PTBMultiLine,
    NEGRA,
}

impl InputFormat {
    fn try_from_str(s: &str) -> Result<InputFormat, Error> {
        let s = s.to_lowercase();
        match s.as_str() {
            "ptb-singleline" => Ok(InputFormat::PTBSingleLine),
            "ptb-multiline" => Ok(InputFormat::PTBMultiLine),
            "negra" => Ok(InputFormat::NEGRA),
            _ => Err(format_err!("Unknown input format: {}", s)),
        }
    }
}

static DEFAULT_CLAP_SETTINGS: &[AppSettings] = &[
    AppSettings::DontCollapseArgsInUsage,
    AppSettings::UnifiedHelpMessage,
];

static INPUT: &str = "INPUT";
static OUTPUT: &str = "OUTPUT";
static IN_FORMAT: &str = "IN_FORMAT";
static OUT_FORMAT: &str = "OUT_FORMAT";

fn parse_args() -> ArgMatches<'static> {
    App::new("lumberjack-convert")
        .settings(DEFAULT_CLAP_SETTINGS)
        .arg(
            Arg::with_name(INPUT)
                .long("input_file")
                .help("Input file")
        )
        .arg(
            Arg::with_name(IN_FORMAT)
                .long("input_format")
                .help("Input formats: [negra, ptb-singleline, ptb-multiline]")
        )
        .arg(Arg::with_name(OUTPUT)
            .long("output_file")
            .help("Output CONLL file")
        )
        .arg(
            Arg::with_name(OUT_FORMAT)
                .long("output_format")
                .help("Output formats: [simple, tuebav2]"),
        )
        .get_matches()
}
