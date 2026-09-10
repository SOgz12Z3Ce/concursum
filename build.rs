use image::ImageFormat;
use serde_json::{Deserializer, Serializer};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    sync::LazyLock,
};
use walkdir::WalkDir;

macro_rules! res_path {
    ($path:literal) => {
        concat!("res/", $path)
    };
}
macro_rules! assets_path {
    ($path:literal) => {
        concat!(res_path!("assets/"), $path)
    };
}
macro_rules! cs_assets_path {
    ($path:literal) => {
        concat!(assets_path!("cs/"), $path)
    };
}
macro_rules! boh_assets_path {
    ($path:literal) => {
        concat!(assets_path!("boh/"), $path)
    };
}

static ETC_STATIC_PATH: &'static str = res_path!("static/");
static CS_CONTENT_PATH: &'static str = cs_assets_path!("content/");
static CS_IMAGES_PATH: &'static str = cs_assets_path!("images/");
static BOH_IMAGES_PATH: &'static str = boh_assets_path!("images/");

static BASE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()));

static OUT_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::new()
        .join(&*BASE_PATH)
        .join("target")
        .join(env::var("PROFILE").unwrap())
});
static OUT_CONTENT_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::new().join(&*OUT_PATH).join("content"));
static OUT_CONTENT_CS_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::new().join(&*OUT_CONTENT_PATH).join("cs"));
static OUT_STATIC_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::new().join(&*OUT_PATH).join("static"));
static OUT_STATIC_IMAGES_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::new().join(&*OUT_STATIC_PATH).join("images"));
static OUT_STATIC_IMAGES_CS_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::new().join(&*OUT_STATIC_IMAGES_PATH).join("cs"));

static UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

static PATTERN_REPLACEMENT: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        (",\n\t\t]", "\n\t\t]"),
        (",\n\t\n\t\n\t\n\t]", "\n\t\n\t\n\t\n\t]"),
        (",\n\t]", "\n\t]"),
        (",\n\n\t\t]", "\n\n\t\t]"),
        (",\n\n\t]", "\n\n\t]"),
        (",\n\n\n\n    ]", "\n\n\n\n    ]"),
        (",\n\n\n    ]", "\n\n\n    ]"),
        (",\n\n\n]", "\n\n\n]"),
        (",\n\n]", "\n\n]"),
        (",\n  \n]", "\n  \n]"),
        (
            ",\n                                 ]",
            "\n                                 ]",
        ),
        (
            ",\n                                ]",
            "\n                                ]",
        ),
        (",\n    ]", "\n    ]"),
        (",\n]", "\n]"),
        (",\n\t\t\t\t}", "\n\t\t\t\t}"),
        (",\n\t\t    }", "\n\t\t    }"),
        (",\n\t\t}", "\n\t\t}"),
        (",\n\t    }", "\n\t    }"),
        (",\n\n\t\t\t\t}", "\n\n\t\t\t\t}"),
        (",\n                    }", "\n                    }"),
        (",\n                  }", "\n                  }"),
        (",\n                }", "\n                }"),
        (",\n              }", "\n              }"),
        (",\n            }", "\n            }"),
        (",\n        }", "\n        }"),
        (",\n    }", "\n    }"),
        (",\n  }", "\n  }"),
        (",\n}", "\n}"),
        ("\"startdes\n\t\t\tcription\"", "\"startdescription\""),
        ("马上就会回来。\n", "马上就会回来。\\n"),
        ("\nand lo,", "\\nand lo,"),
        ("{id:", "{\"id\":"),
        ("            id:", "            \"id\":"),
        ("isHidden:", "\"isHidden\":"),
        ("additive:", "\"additive\":"),
        ("description:", "\"description\":"),
        ("isAspect:", "\"isAspect\":"),
        ("label:", "\"label\":"),
        ("spec:", "\"spec\":"),
        ("defaultcard:", "\"defaultcard\":"),
        ("resetonexhaustion:", "\"resetonexhaustion\":"),
        ("comments:", "\"comments\":"),
        ("drawmessages:", "\"drawmessages\":"),
        ("influencemoth:", "\"influencemoth\":"),
        ("fragmentsecrethistories:", "\"fragmentsecrethistories\":"),
        ("rumour:", "\"rumour\":"),
        ("influencewinterc:", "\"influencewinterc\":"),
        ("influencegrail:", "\"influencegrail\":"),
        ("vitality:", "\"vitality\":"),
        ("fragmentsecrethistoriesb:", "\"fragmentsecrethistoriesb\":"),
        ("influenceknock:", "\"influenceknock\":"),
        ("influenceheart:", "\"influenceheart\":"),
        ("restlessness:", "\"restlessness\":"),
        ("influenceknockc:", "\"influenceknockc\":"),
        ("fragmentsecrethistoriesc:", "\"fragmentsecrethistoriesc\":"),
        ("glimmering:", "\"glimmering\":"),
        ("erudition:", "\"erudition\":"),
        ("fascination:", "\"fascination\":"),
        ("favour:", "\"favour\":"),
        ("influencelanternc:", "\"influencelanternc\":"),
        ("influenceheartc:", "\"influenceheartc\":"),
        ("influenceknocke:", "\"influenceknocke\":"),
        ("influenceforgec:", "\"influenceforgec\":"),
        ("influenceedgee:", "\"influenceedgee\":"),
        ("influencegrailc:", "\"influencegrailc\":"),
        ("influencewintere:", "\"influencewintere\":"),
        ("fragmentsecrethistoriesd:", "\"fragmentsecrethistoriesd\":"),
        ("eruditionplus:", "\"eruditionplus\":"),
        ("influencegraile:", "\"influencegraile\":"),
        ("influencehearte:", "\"influencehearte\":"),
        ("influenceforgeg:", "\"influenceforgeg\":"),
        ("fragmentsecrethistoriese:", "\"fragmentsecrethistoriese\":"),
        ("ingredientforgef:", "\"ingredientforgef\":"),
        ("influencelanterne:", "\"influencelanterne\":"),
        ("influenceknockg:", "\"influenceknockg\":"),
        ("influencemothe:", "\"influencemothe\":"),
        ("scholarvak:", "\"scholarvak\":"),
        ("influencelanterng:", "\"influencelanterng\":"),
        ("influencegrailg:", "\"influencegrailg\":"),
        ("ingredientgraild:", "\"ingredientgraild\":"),
        ("fragmentsecrethistoriesf:", "\"fragmentsecrethistoriesf\":"),
        ("influencewinterg:", "\"influencewinterg\":"),
        ("dread:", "\"dread\":"),
        ("\n-", "\\n-"),
        ("\n\t-", "\\n\\t-"),
        ("\t\t\t\t\"\n\t}", "\\t\\t\\t\\t\"\n\t}"),
        ("\t\t\t\"\n\t\t}", "\\t\\t\\t\"\n\t\t}"),
        ("leaks\n", "leaks\\n"),
        ("unplayable)\n", "unplayable)\\n"),
        ("changes\n", "changes\\n"),
        ("hints\n", "hints\\n"),
        ("keyboard\n", "keyboard\\n"),
        ("localisation\n", "localisation\\n"),
        ("(beta)\n", "(beta)\\n"),
        ("typos\n", "typos\\n"),
        ("biz\n", "biz\\n"),
        ("Legacy\n", "Legacy\\n"),
        ("correctly once again\n", "correctly once again\\n"),
        ("upload\n", "upload\\n"),
        ("work once again\n", "work once again\\n"),
        ("Update again\n", "Update again\\n"),
        ("typos)\n", "typos)\\n"),
        ("credits\n", "credits\\n"),
        ("longer.\n", "longer.\\n"),
        ("setting works again.\n", "setting works again.\\n"),
        ("life).\n", "life).\\n"),
        ("braces.\n", "braces.\\n"),
        ("instantly.\n", "instantly.\\n"),
        ("languages.\n", "languages.\\n"),
        ("main menu.\n", "main menu.\\n"),
        ("files.\n", "files.\\n"),
        ("the game.\n", "the game.\\n"),
        ("button.\n", "button.\\n"),
        ("out.\n", "out.\\n"),
        ("possible.\n", "possible.\\n"),
        ("when.\n", "when.\\n"),
        ("cash.\n", "cash.\\n"),
        ("screen.\n", "screen.\\n"),
        ("hauntings.\n", "hauntings.\\n"),
        ("more ?!\n", "more ?!\\n"),
        ("etc).\n", "etc).\\n"),
        ("E.\n", "E.\\n"),
        ("slots once again.\n", "slots once again.\\n"),
        ("confrontation.\n", "confrontation.\\n"),
        ("there.\n", "there.\\n"),
        ("scale.\n", "scale.\\n"),
        ("ends.\n", "ends.\\n"),
        ("sorted.\n", "sorted.\\n"),
        ("\nThis", "\\nThis"),
        ("as:\n", "as:\\n"),
        ("once.\n", "once.\\n"),
        ("working anymore.\n", "working anymore.\\n"),
        ("dialog.\n", "dialog.\\n"),
        ("logs.\n", "logs.\\n"),
        (
            "consumed on influence upgrade and subversion.\n",
            "consumed on influence upgrade and subversion.\\n",
        ),
        ("chain.\n", "chain.\\n"),
        (
            "correctly on influence upgrade and subversion.\n",
            "correctly on influence upgrade and subversion.\\n",
        ),
        ("new game.\n", "new game.\\n"),
        ("fixed.\n", "fixed.\\n"),
        ("restored.\n", "restored.\\n"),
        ("initializing.\n", "initializing.\\n"),
        ("display.\n", "display.\\n"),
        ("speeds.\n", "speeds.\\n"),
        ("situations.\n", "situations.\\n"),
        ("locs.\n", "locs.\\n"),
        ("Starvation verb.\n", "Starvation verb.\\n"),
        ("Collection.\n", "Collection.\\n"),
        ("framerates.\n", "framerates.\\n"),
        ("failing.\n", "failing.\\n"),
        ("tap.\n", "tap.\\n"),
        ("work once again.\n", "work once again.\\n"),
        ("quirks.\n", "quirks.\\n"),
        ("text.\n", "text.\\n"),
        ("mutations.\n", "mutations.\\n"),
        ("correctly once again.\n", "correctly once again.\\n"),
        ("Exile.\n", "Exile.\\n"),
        ("description.\n", "description.\\n"),
        ("leaks.\n", "leaks.\\n"),
        ("closed.\n", "closed.\\n"),
        ("descriptions.\n", "descriptions.\\n"),
        ("fit.\n", "fit.\\n"),
        ("timers.\n", "timers.\\n"),
        ("draws).\n", "draws).\\n"),
        ("Sister.\n", "Sister.\\n"),
        ("colour.\n", "colour.\\n"),
        ("Time verb.\n", "Time verb.\\n"),
        ("dramatically again.\n", "dramatically again.\\n"),
        ("sorry!\n", "sorry!\\n"),
        ("requirements.\n", "requirements.\\n"),
        ("open.\n", "open.\\n"),
        ("faded once again.\n", "faded once again.\\n"),
        ("clicked.\n", "clicked.\\n"),
        ("settings menu.\n", "settings menu.\\n"),
        ("circumstances.\n", "circumstances.\\n"),
        ("empty.\n", "empty.\\n"),
        ("were.\n", "were.\\n"),
        ("alphabetical).\n", "alphabetical).\\n"),
        ("name.\n", "name.\\n"),
        ("properly.\n", "properly.\\n"),
        ("defined.\n", "defined.\\n"),
        ("current verb.\n", "current verb.\\n"),
        ("work.\n", "work.\\n"),
        ("$plus/$minus work again.\n", "$plus/$minus work again.\\n"),
        ("levers.\n", "levers.\\n"),
        ("one.\n", "one.\\n"),
        ("disappear.\n", "disappear.\\n"),
        ("resolutions.\n", "resolutions.\\n"),
        ("VARLEY)\n", "VARLEY)\\n"),
        ("Passions.\n", "Passions.\\n"),
        ("halved.\n", "halved.\\n"),
        ("Mansus.\n", "Mansus.\\n"),
        ("Exiles anymore.\n", "Exiles anymore.\\n"),
        ("group.\n", "group.\\n"),
        ("interactions once again.\n", "interactions once again.\\n"),
        ("without \\\"\\\".\n", "without \\\"\\\".\\n"),
        ("lists.\n", "lists.\\n"),
        ("only).\n", "only).\\n"),
        ("properties:\n", "properties:\\n"),
        ("\t- 'Ids'", "\\t- 'Ids'"),
        ("code.\n", "code.\\n"),
        ("\tA simple", "\\tA simple"),
        ("\nKeep", "\\nKeep"),
        ("\nIn", "\\nIn"),
        ("\nAnd", "\\nAnd"),
        ("\t- Hardened against some", "\\t- Hardened against some"),
    ])
});

static HASH_PATTERNS: LazyLock<HashMap<&'static str, Vec<&'static str>>> = LazyLock::new(|| {
    HashMap::from([
        (
            "02a996afd3a77a8413080d9a8acb6e11d28e3bfa991e8e9ac80fae7114ed0a55",
            vec![
                ",\n\t\t]",
                ",\n}",
                "\n-",
                "\n\t-",
                "\t\t\t\t\"\n\t}",
                "\t\t\t\"\n\t\t}",
                "leaks\n",
                "unplayable)\n",
                "changes\n",
                "hints\n",
                "keyboard\n",
                "localisation\n",
                "(beta)\n",
                "typos\n",
                "biz\n",
                "Legacy\n",
                "correctly once again\n",
                "upload\n",
                "work once again\n",
                "Update again\n",
                "typos)\n",
                "credits\n",
                "longer.\n",
                "setting works again.\n",
                "life).\n",
                "braces.\n",
                "instantly.\n",
                "languages.\n",
                "main menu.\n",
                "files.\n",
                "the game.\n",
                "button.\n",
                "out.\n",
                "possible.\n",
                "when.\n",
                "cash.\n",
                "screen.\n",
                "hauntings.\n",
                "more ?!\n",
                "etc).\n",
                "E.\n",
                "slots once again.\n",
                "confrontation.\n",
                "there.\n",
                "scale.\n",
                "ends.\n",
                "sorted.\n",
                "\nThis",
                "as:\n",
                "once.\n",
                "working anymore.\n",
                "dialog.\n",
                "logs.\n",
                "consumed on influence upgrade and subversion.\n",
                "chain.\n",
                "correctly on influence upgrade and subversion.\n",
                "new game.\n",
                "fixed.\n",
                "restored.\n",
                "initializing.\n",
                "display.\n",
                "speeds.\n",
                "situations.\n",
                "locs.\n",
                "Starvation verb.\n",
                "Collection.\n",
                "framerates.\n",
                "failing.\n",
                "tap.\n",
                "work once again.\n",
                "quirks.\n",
                "text.\n",
                "mutations.\n",
                "correctly once again.\n",
                "Exile.\n",
                "description.\n",
                "leaks.\n",
                "closed.\n",
                "descriptions.\n",
                "fit.\n",
                "timers.\n",
                "draws).\n",
                "Sister.\n",
                "colour.\n",
                "Time verb.\n",
                "dramatically again.\n",
                "sorry!\n",
                "requirements.\n",
                "open.\n",
                "faded once again.\n",
                "clicked.\n",
                "settings menu.\n",
                "circumstances.\n",
                "empty.\n",
                "were.\n",
                "alphabetical).\n",
                "name.\n",
                "properly.\n",
                "defined.\n",
                "current verb.\n",
                "work.\n",
                "$plus/$minus work again.\n",
                "levers.\n",
                "one.\n",
                "disappear.\n",
                "resolutions.\n",
                "VARLEY)\n",
                "Passions.\n",
                "halved.\n",
                "Mansus.\n",
                "Exiles anymore.\n",
                "group.\n",
                "interactions once again.\n",
                "without \\\"\\\".\n",
                "lists.\n",
                "only).\n",
                "properties:\n",
                "\t- 'Ids'",
                "code.\n",
                "\tA simple",
                "\nKeep",
                "\nIn",
                "\nAnd",
                "\t- Hardened against some",
            ],
        ),
        (
            "02fbd1122b1272c8c1d620a2c41cf4d9abcdaab586f329d84698bcedcda8cab0",
            vec![",\n\t\t\t\t}"],
        ),
        (
            "0733ff54a9b9cbeddd710784fb373a91afe125a33ae303c71a6e5b84f64f6cb9",
            vec![",\n\n]"],
        ),
        (
            "09eb3d13191f4ed91e7b0f5d038619b47a3c3d339f9591417d08133f5d3b630f",
            vec![",\n                }"],
        ),
        (
            "0bd0b569cd08089f50315960c0461aa5f03fddeefb61d83b7febfe5385458029",
            vec![",\n        }"],
        ),
        (
            "0c6ca056a0e7132e6005e10aefadeca1dc007a5e580abbc48e086472498ac32c",
            vec![",\n            }"],
        ),
        (
            "0ce7bdab218124ca3572c04e7faabfe9d60847b665855073d208c99134407d1d",
            vec![",\n\n\n    ]", ",\n    }"],
        ),
        (
            "0d4cba4e867d486ca19e17ee43197d6ca8338e85fbfc02119a3c12108dc2fc99",
            vec![",\n                  }", ",\n        }"],
        ),
        (
            "0dd29a4603b9f0b1d11429b7126b4e166ef39555fc0887f8b5ca6d140566b843",
            vec![",\n                                ]", ",\n        }"],
        ),
        (
            "0e33d6ed36c69d70120b6f7ad7226d1dc131a832f2b7dd7fb7ddc06c1f702d54",
            vec![",\n        }"],
        ),
        (
            "0fbcc35bb9b3b2af77f03c14c2a850bc6c7a777c4893bcd9d6eed6490c4c5523",
            vec![",\n                }"],
        ),
        (
            "1522405cf017515ad058f3d8db981e20b5c8ce1dd0c1b2dc9e772bfb13ad24e5",
            vec![",\n                }"],
        ),
        (
            "19428b9e34a44966109cc726af023ce4a108aa735aed65923d71fda6a62ee1a4",
            vec![",\n\n]"],
        ),
        (
            "19db8b51710980231bb9898f4ed33621df765417671dcfa6d7c6a6671c87f057",
            vec![",\n\n\t]", ",\n\t\t}"],
        ),
        (
            "1a1f32be401d1d99dcd4b1c8579b6b9fa09324d390012a067cdb87ef8f6cc96f",
            vec![",\n\n\t\t\t\t}"],
        ),
        (
            "1bf2c8e45f85790d6937d47d634faa993b44f3ca09ebd5720a76805ef6f7730b",
            vec![",\n  \n]"],
        ),
        (
            "1ce2bb790cb595c6fcf1ccd731af36300daaa32ebfbb7ec001010ed6410dd245",
            vec![",\n\n\t\t\t\t}"],
        ),
        (
            "1ed7419daee8f79d05ec6c1e15628c47f2380e7781117b54d6396ba644e297e1",
            vec![",\n            }"],
        ),
        (
            "1ef249fec406d549e5cc152b0496d2fec4461f1ad0b89cc728ef5574c73209a1",
            vec![",\n            }"],
        ),
        (
            "1f314669d1486a4322cff65fe9c460dce7c5f0bc200fa305fb581ca220ccd5a4",
            vec![",\n        }"],
        ),
        (
            "225fdf4d9d7966b87499ea667e42acc0f88fbaa075463e612a5c12bf54dac14b",
            vec![",\n\t\t\t\t}"],
        ),
        (
            "239cd80507cd89549f74b717cf5e9bbbbff37942e3a785d7fb75301658886f27",
            vec![",\n    ]", ",\n                }"],
        ),
        (
            "23cc1389c76209f767abf9fd5806f9f192ca29df1406ee4b3d7b011633542fc3",
            vec![",\n        }"],
        ),
        (
            "23dcd14fe1e22d5d29b724bd83a3b936fae6abf7054da7b301433b91266e23f8",
            vec![",\n\n]"],
        ),
        (
            "27d98b0b13856d42011ff8ec5910d7ba7b15eeb238b16a9a77d9eb8788620c99",
            vec![",\n\n\t\t\t\t}"],
        ),
        (
            "2803e0071a99c37f78ab5e7c0ddee5db8e259df04e4736780e7311c30773b983",
            vec![",\n\t\t\t\t}"],
        ),
        (
            "28ad577bbe12c0524af0bcb167c6b8faba0a7ee992e314ce9e3e60d45e9aeba3",
            vec![",\n                }"],
        ),
        (
            "2c8184ea23b309b02c7be095bfa24dd414cd6414683e65ff55854c27e96f8647",
            vec![",\n        }"],
        ),
        (
            "2dad73f8d114f310b01c498dfba7112b8ab3004c7fb3ee97307952241118d75a",
            vec![",\n\n]"],
        ),
        (
            "2e2028d00652c4e1d368d2b32b800452b97a1926a523442dd129ebe53c8d1044",
            vec![",\n    ]"],
        ),
        (
            "2f26cf19ee8f0547881fa6ea1e507c0ed36a6135214760e4b04a8afbe2f4bd70",
            vec![",\n        }"],
        ),
        (
            "2f374ca8b492dbd0c7584db21823f412f43d4c014a28040ae8317ff733b518d6",
            vec![",\n\n\n    ]", ",\n    }"],
        ),
        (
            "306d3b263f917fe880565847a9a17b59ec047f2c0e5af334a2612b02dedf19b8",
            vec![",\n            }"],
        ),
        (
            "346a8ff7c1db5364eba59559721465e0fc8e7ffca516e1370ca5c331bbc8b9a6",
            vec![",\n        }"],
        ),
        (
            "346bc3f009b4012afae8ca297a019670fe6f306deaaddf8d1f31c3eb92900659",
            vec![",\n\n]"],
        ),
        (
            "35599d9d8c1464c0a7513db41d01ebc8569fbe89d355946878b8e931c7bd8a87",
            vec![",\n\n]"],
        ),
        (
            "362cb4838fe542232816d3523169aa5a1bd8229227a7d11fc7c6177c180fafe8",
            vec![",\n\n]"],
        ),
        (
            "37ead6fe7f5ee0cc17264ff0fc4420229d8fa382cf5885d97aa52af8bcd443c6",
            vec![",\n                }"],
        ),
        (
            "39207cd0ec0bdde4e71f00cf4e233f4466c5b92832c04ef3e9fd7e36cb01ee30",
            vec![",\n\t\t\t\t}"],
        ),
        (
            "39def76b06104396a9a6c63130cfabfa2f1f3428b266f6b6c4219fda3f595fdc",
            vec![",\n                }"],
        ),
        (
            "3cfeb6ffbb11f14c5692de2828f3be288ff4646bbed63151891d0cffbb827e61",
            vec![",\n                }"],
        ),
        (
            "3e5c6ef648e8684eec4c5358d6d65cb712fffde24db7d7966b4c57973a88b8be",
            vec![",\n\n]"],
        ),
        (
            "3ef61176b10a2c8a10200a9b45e334bf996f12ea5196e16ccd1f958e8f0e9644",
            vec![",\n\n]"],
        ),
        (
            "40b875c1c5ceb7c8ed10197401ea80c33800a863e30124f3947ef57a51d123e2",
            vec![",\n    ]"],
        ),
        (
            "413f3c5dfd4d71f0a273423f98283f80cf1ac1f2ae8afcca4fda40bc430e8ece",
            vec!["\"startdes\n\t\t\tcription\""],
        ),
        (
            "42f7c3b4c7d9431f01f5572e05763a76a9ed9bbd6eff9c33d37cbf31035c7d68",
            vec!["additive:"],
        ),
        (
            "473dd87351cc068cdfe7e12a60f4bdd90dbf0e63ef7f4e753d3e7187f868b618",
            vec!["additive:"],
        ),
        (
            "47b9f00305e8a717c60c54dccc3be3b93533f47eed25c535b2dc1043ebda1325",
            vec![",\n            }"],
        ),
        (
            "485f25ed21f30a003daab07470efd96b5192a04e764094b54744c9e6e3ba3125",
            vec![",\n            }"],
        ),
        (
            "4a912ec123133537d2bd821471a96ba33ccc8682b774e11b4b8f44200b019453",
            vec![",\n            }"],
        ),
        (
            "4ec903234c91bc6bc24fa74cfc6f277c13274b7832071b1e1ceaa14d6bd7b360",
            vec![",\n        }"],
        ),
        (
            "52a9b757ce304bd70b2e52535080c8347a70f45b958f5e27ad7ca6653ccdc30d",
            vec![",\n            }"],
        ),
        (
            "52b32473b3f081dbab65b1bd1b5e2342e3c732a8dd91498063635a646d46899a",
            vec![",\n        }"],
        ),
        (
            "5660a608dd3af066c386e51bab5067bf1cfbbd93be3e3367aae8a42f60faa55e",
            vec![",\n    ]"],
        ),
        (
            "5893f022dc0a2ac53e09fe2a93bb83083dfbf4e25b090ab9637b271d8ed0bc73",
            vec![",\n    ]"],
        ),
        (
            "5ae714a5f554d08e44d0ca266541db66089b674d3c73bd7ab0e1a0e60213544e",
            vec![",\n  \n]"],
        ),
        (
            "5ceb55927882e254cec29af3bb1989030b745bef7f286757114b639d9df321cf",
            vec![",\n                                 ]"],
        ),
        (
            "5f66d29e70d65d435989d0836b2a9838476b3e60e13b440b3f3c1bddb4b0a8a4",
            vec![",\n\n\n]"],
        ),
        (
            "5f7c431ddf3b78d5763e1cb9aed58c2de5f02adb8951a5b4171a5eb256eb1447",
            vec![",\n        }"],
        ),
        (
            "608520c819f70f0c5aa03e8a0501ae1fbf0fe8486b9f3bcdaa711ed772332b85",
            vec![",\n        }"],
        ),
        (
            "6228fba4030b491dc79f2ebd0942eaf5e6adea217a744b122e7fefc313704ea4",
            vec![",\n            }"],
        ),
        (
            "62d3c9a3a9274f0b4d5019f03af63b9dc4b3c97cce7d0e79a6b1718dcc3bb6ea",
            vec![",\n\n\n]"],
        ),
        (
            "66e7d7b4691196bb74d442f41737c47739669a14f8b00e68eeba04e79d77d8c6",
            vec![",\n            }"],
        ),
        (
            "692a73f4c24a226a9adbd6a393ce101d3e588e4d3dc7a5cb489c737701b61df1",
            vec![",\n        }"],
        ),
        (
            "6b3e55bd303448b520ebe429f30004c2fba405077e792eccba09f5cf41275773",
            vec![",\n            }"],
        ),
        (
            "6bde1600a038a3ab76e748eef71f6fb115d89ca099e9fab21a3f2aae482e94bc",
            vec![",\n            }"],
        ),
        (
            "6cdfbfa7199203497fbad2f04f248c9b08ad56d186e72b686d98527c427a934f",
            vec![",\n    ]", ",\n                }"],
        ),
        (
            "6eb59f1354a671c34194096d6a6d5c2ba897253066af731487f200e6c179b08f",
            vec![",\n                                ]", ",\n        }"],
        ),
        (
            "6f4640ded6518272514681a1659f035fb551d17fc1103438d9b4874dcbe0bbb7",
            vec![",\n    ]", ",\n        }"],
        ),
        (
            "717513ed9612dc43d15b3bb930a4e57a3a66c9035b99ebc2a12b2ad32fe258a3",
            vec![",\n\n]"],
        ),
        (
            "71d775e82aca5d7f5ef6c2ec669322c93d91ac6c237241fef87369eb6fa69568",
            vec![",\n\n\n\n    ]", ",\n\t\t    }", ",\n\t    }", ",\n}"],
        ),
        (
            "727a33e6dd266e38f28f626c17713328f00ecded982490c25ca20618a483b0d5",
            vec![",\n\n]"],
        ),
        (
            "72ee9d3e5d9f8a54127a83d9e18b81c85c2769a94cf34f07c2c4db8a1935a894",
            vec![",\n            }"],
        ),
        (
            "74289109fbb37ecfbe0add0688c5f3f07db9f8a73de90c8232699e8f490b8d99",
            vec![",\n\n]"],
        ),
        (
            "7494fb264d0727728221eca9b3df814797be8076478d8b6e992e22cc803b6b38",
            vec![",\n        }"],
        ),
        (
            "7ee500bf74931c001346f5f68218665ecb8809a441ae705e40816d380fd1db85",
            vec![",\n            }", ",\n        }"],
        ),
        (
            "81047524c8c2265ebe02cc42178d3aae144f5bbb9dbe0ad0124e505f10fd98e4",
            vec![",\n            }"],
        ),
        (
            "822054d2898a87502d10cd6cee58ebf8e7fc2597610fa385b372a046017ab209",
            vec![",\n  \n]"],
        ),
        (
            "83ed6dcc8fa0a21a7155541bdb26575ace4c7d473d2f5692f893c6cb6991c2ac",
            vec![",\n                                 ]"],
        ),
        (
            "86a06922c7e81bcc685b426fbda3a45d222f386cc511239150310113cdd3b137",
            vec![",\n            }", ",\n        }"],
        ),
        (
            "8a41fc6a0985c035084861f4abd8aa033b44b2cc5bea58271d9d4c0ad823fb85",
            vec![",\n]"],
        ),
        (
            "8ad83dcf29ae47a88836dc6c60dac1175ea3f789ba17cde5ced81e17d6e554cd",
            vec![",\n            }"],
        ),
        (
            "8d908062b08ba1cfdc11c50397563551ae9d67690da10f33c70d505c64f4100b",
            vec![",\n    ]", ",\n                    }"],
        ),
        (
            "8eb43fbe2bd141a8c10eef04d07b0513887ce06dee5f94b2c9bab2a787a216ab",
            vec![",\n    ]"],
        ),
        (
            "9158d066becf07a66be6a2fd15588fe15da7d65a6fc7850c61d07c7205645067",
            vec![",\n            }", ",\n        }"],
        ),
        (
            "91fc5013c9dc637c3547d27a56377d8c9331dcf59ff08ecd8458a67313892dfe",
            vec![",\n            }"],
        ),
        (
            "937a83b8901c39acc1bce7dbfc86d444398f1c1a02912fe8171695940766eeba",
            vec![",\n    ]"],
        ),
        (
            "943dac6438378c3e899bbcafdeb6200769603d92323f305426dcfc410783ee28",
            vec![",\n    ]", ",\n        }"],
        ),
        (
            "9448a5da61f06b13fce6fe7296d967016b7f2a5475735cdd68ff7067a6912086",
            vec![",\n\n]"],
        ),
        (
            "95b0453a2063c6d853654ef9f6896010ee1882518dbd8e891ed52d90fee56d15",
            vec![",\n\n]"],
        ),
        (
            "9617c130b5513e2d91f60f64d9456c2e3cbffc31047736ae355ad616f6e3178c",
            vec![",\n        }"],
        ),
        (
            "962b9b806dddb7614827aa3b6695fc71be50899dd45889fd9aff62f55faa929f",
            vec![",\n\n\t\t]", ",\n}"],
        ),
        (
            "98529b504f56847011c10858bb386eefe66fe6d22aa3c17a9b8ced2f068e56b3",
            vec![",\n            }", ",\n        }"],
        ),
        (
            "9870e7295625c94e31bde390aa76038e5a3744e29f3b3c7d76f3ef3a93afe579",
            vec![",\n\n]"],
        ),
        (
            "98dec6705503e576c6a3da5678d06fa59966e5f6d5fe56e264316ebade293c96",
            vec![",\n            }"],
        ),
        (
            "9907f4ba82108b1846cd882bee1ca9ff2b74d2e01b1e7189a281e3c302abcabc",
            vec!["additive:"],
        ),
        (
            "9b006d7b6163000ba4ae631266618bb09d3259126faac1aa85de35192795a125",
            vec![",\n            }"],
        ),
        (
            "9b82af177a90f9027cb9a561d50942a9ce5ea094c4ed809ed7b93e9c02b56868",
            vec![",\n    ]", ",\n                }", ",\n        }"],
        ),
        (
            "9c1062cb857d64aac8a207e878b1e5f3017da402a0d94fff211fc7f53a51aa49",
            vec![
                "{id:",
                "            id:",
                "isHidden:",
                "description:",
                "isAspect:",
                "label:",
            ],
        ),
        (
            "9f4f96e1cd887b6e42be72347257e66d13643cd093760348b023fda3a3b0154b",
            vec![",\n        }"],
        ),
        (
            "a117bc2ac7512b1e509ff307b623e6542d15b7b8ae7aafd3e514711afc86070d",
            vec![",\n    ]"],
        ),
        (
            "a25e3a75661d689a13b0af06ab93c5f1483915697bc6d700e1f4b3e2fbdf13b9",
            vec![",\n        }"],
        ),
        (
            "abc3c58720cf53ea34321bb05c5ba8d42fca015b850df122d0ee57be37840332",
            vec![",\n        }"],
        ),
        (
            "abcb1675dc5d97df4b16c12e64c075c131b31d7cd55bd0b191b9f9ecab68bbe4",
            vec![",\n\n]"],
        ),
        (
            "adedbbf7ba2edf5f170c6e5382d84e5dadd12731f3af9072ea57ad78094b6a54",
            vec![",\n    }"],
        ),
        (
            "ae1d09dd8f3ce35392e20acce10b9c301322c889abfa191aa5261a0dc7f639db",
            vec![",\n\n]"],
        ),
        (
            "afa6b0b65f612a9c85983cf24c0f88ff3346d714638e3e6779213d9204628b45",
            vec![",\n    ]", ",\n                    }"],
        ),
        (
            "b1bc3109e35d3f3b4fb723ca184a27e85d72df3eb1d8fb9b5de911a12eeea080",
            vec![",\n        }"],
        ),
        (
            "b1d0f197675f3edeff5a966779e0b0014853e0b84e501a8624007061fe3d7fd1",
            vec![",\n\n]"],
        ),
        (
            "b2828af4c52b2cf03e45da07d3c3b43754cbf1ee3eaef677a613c18ec8a7a8e0",
            vec![",\n\n]"],
        ),
        (
            "b8b0e80f8915ef044b1bdb56d2f2009c7b314795ff65b68ee6a3930063f498f8",
            vec![",\n\n]"],
        ),
        (
            "b98040d1735e8a107fa61e75ac395c231f9037ae35b389c8d34f88e0ab7c04c6",
            vec![",\n            }"],
        ),
        (
            "bab19a3fb4d8ebaccc3fb047a192338e4502f7901ac37476240a9cdc487db7da",
            vec![",\n        }"],
        ),
        (
            "bd57e4b32de060b6f97168b537f197deed8ec9b99e1f1617cb80fa3f83038f94",
            vec![",\n            }"],
        ),
        (
            "bee3da16c8a6dfb0e2a54709bacb785a5525d979652eb14092e08520fe553f1d",
            vec![",\n            }"],
        ),
        (
            "bf7d9ac8b9147547b3f35e18dab9c8ea7e3620ffe609dbc0aa0798c1495d4c7b",
            vec![",\n\n\n]"],
        ),
        (
            "c04f9db1f8390de1efc7c60a0980877df698f47a0c3918563a080c083669f4b5",
            vec![",\n  \n]"],
        ),
        (
            "c1ffd669c4787cb9fe4b8388145170400d2892096f10c261222570e850b27b52",
            vec![",\n        }"],
        ),
        (
            "c3976ad5fbd0f6707c5cdcfb849ffbeba381f00151a6dbec02c98cb24a5bdd0c",
            vec![",\n        }"],
        ),
        (
            "c6d0dc6fbbc2d0e4575b2cd83b79eff8560a04d13c291aa7010fa1daf5cb9b2e",
            vec![",\n                }"],
        ),
        (
            "c8fc24cc683f15ba366097b0a1b1ecec44d80eeb31ade1ff70afcf52d4ecccd8",
            vec![",\n\n\n    ]", ",\n    }"],
        ),
        (
            "cc946f3fac334918bc15366979afd8b3eede878772fbbf86418fec15d0c76906",
            vec![",\n\n]"],
        ),
        (
            "cdd31fc64cfcde5236ef58318fa006fb175d54d130670a042b5f7bfc96c60fc4",
            vec![",\n    ]", ",\n            }"],
        ),
        (
            "cde59119543977368cfcf7107e018cd988fa3d5a8e72aaf49c4d92e0156943b2",
            vec![",\n\n\n]"],
        ),
        (
            "ce99af16836a8fbe2ad4afc737086a0df0794e12e187bae6d41e8f3f504ec09b",
            vec![",\n                                 ]"],
        ),
        (
            "cee8f568811a4fb1b1cc30bec1915a7fb7510dc33d93b1357bd7d7813f684c9c",
            vec![",\n            }"],
        ),
        (
            "d0b284412e71a2a57b2b5d12378197464b2dfe3a0d1a775efdc1be2766c421f2",
            vec![",\n\n]"],
        ),
        (
            "d28bde449e520cd10001affbbab03af45ab126e9df617b04215c1a2dac56ff97",
            vec![",\n    ]", ",\n        }"],
        ),
        (
            "d58e3a8dad2e1e609e54365a3891a24938741efe53e0470463fca158a5a78a6e",
            vec![",\n                }"],
        ),
        (
            "d609ec58fef98be1097684122b35f414468c363dd9a4b679f9307a2e180312e3",
            vec![",\n\t]"],
        ),
        (
            "d9be7a282bd8554b9be70192c176e518e5a2188bede806db67fe531fc933330c",
            vec![",\n            }", ",\n        }"],
        ),
        (
            "d9c9f5885ae68ebbc0347cd7bebcbeae9da7c1dbe398c71f0d3cf58b2b193be8",
            vec![",\n                                ]", ",\n        }"],
        ),
        (
            "db7a3db6e501048464d7d46c5ecb3ea4458b610b74c8a66a753ff97c7ec2a01b",
            vec![",\n    ]", ",\n                    }"],
        ),
        (
            "def7565c7ab358e89b17c893f3ed220cdd611ef4ad54a0b764a1722dbec2bcc4",
            vec![",\n    ]", ",\n                }"],
        ),
        (
            "df8636438907386363d7833c829e120999454f6fa609ef68b18b87c017f291bc",
            vec![",\n        }"],
        ),
        (
            "e3448466e62e4af7d90d88bb180c4a7a38c64c8708d97af07c0d4c087c9ae72a",
            vec!["马上就会回来。\n"],
        ),
        (
            "e50ff850baa1ab9b6503edad8fc20270aabd854b80c6c4e716670ab17362a3d3",
            vec![",\n        }"],
        ),
        (
            "e61c333162fad53d06a487a7fd282ad74b53f0eb4c000792d79ac6cfda355982",
            vec![",\n            }"],
        ),
        (
            "e6bec66959e1b95253a59e4ef6ade4cfdcae649ae71a0c853babe20d2e251c98",
            vec![",\n\n\t\t]", ",\n}"],
        ),
        (
            "e9f0d35392eee9fb30d5db270822687150a916cfd903ea277817755939020a01",
            vec![
                ",\n\t\t]",
                ",\n\t\n\t\n\t\n\t]",
                ",\n\t\t}",
                "\nand lo,",
                "{id:",
                "spec:",
                "defaultcard:",
                "resetonexhaustion:",
                "comments:",
                "drawmessages:",
                "influencemoth:",
                "fragmentsecrethistories:",
                "rumour:",
                "influencewinterc:",
                "influencegrail:",
                "vitality:",
                "fragmentsecrethistoriesb:",
                "influenceknock:",
                "influenceheart:",
                "restlessness:",
                "influenceknockc:",
                "fragmentsecrethistoriesc:",
                "glimmering:",
                "erudition:",
                "fascination:",
                "favour:",
                "influencelanternc:",
                "influenceheartc:",
                "influenceknocke:",
                "influenceforgec:",
                "influenceedgee:",
                "influencegrailc:",
                "influencewintere:",
                "fragmentsecrethistoriesd:",
                "eruditionplus:",
                "influencegraile:",
                "influencehearte:",
                "influenceforgeg:",
                "fragmentsecrethistoriese:",
                "ingredientforgef:",
                "influencelanterne:",
                "influenceknockg:",
                "influencemothe:",
                "scholarvak:",
                "influencelanterng:",
                "influencegrailg:",
                "ingredientgraild:",
                "fragmentsecrethistoriesf:",
                "influencewinterg:",
                "dread:",
            ],
        ),
        (
            "eb26a9d530f6fb33a49636cc5a5f947c978247db5deb60d5f4926eafbcfb873c",
            vec![",\n\n\t]", ",\n\t\t}"],
        ),
        (
            "ef26215307b36d8e9c73ab498898f59ff7bb4315b3a665aa9912612d96a32ec1",
            vec![",\n  \n]"],
        ),
        (
            "f15bcfda6a83c2a8cca5fe59cb854ef3ab09277365a0403b02e545b8025f32ad",
            vec![",\n            }", ",\n        }"],
        ),
        (
            "f1d4ea837521dbc954751d630013d6781769b51bb4bc901b3a7631d3646d84f7",
            vec![",\n\n]"],
        ),
        (
            "f29b93c00105984e7ecd5a26c96bc4b58fa657eec5793bc9f7ccb19c7a9cac54",
            vec![",\n        }"],
        ),
        (
            "f3474c919c14d5080010fd12335ebbc87874ff98f4a15fc9b3fee41e3c429b8d",
            vec![",\n\n\n]"],
        ),
        (
            "f3cc619ee425751bb7d420a7f139b7003d5b252048131139bb0e63126fa138c0",
            vec![",\n            }"],
        ),
        (
            "f5c85afa9b37e0c026d3037b57cb88f1a26f82aa0039bf60ea2966666407645a",
            vec![",\n\n\t]", ",\n\t\t}"],
        ),
        (
            "f64ae2032ce9cc6ad81af455027cf1c382ca017498120ed75ca04d76f8e8691b",
            vec![",\n        }"],
        ),
        (
            "f6e0ac21fdabfb3e3c86a593358c1afb81076823ede0a89975e562b1d34ee622",
            vec![",\n        }"],
        ),
        (
            "f95f4fe8266087114d85a21aeb84dd7f81d79eaa614d23e1fe162d2c1f415635",
            vec![",\n\n]", ",\n              }", ",\n  }"],
        ),
        (
            "f9b56c06115fd8e8548fb368e9a8e039f4ca25afe2809840a9d5802367234bf2",
            vec![",\n        }"],
        ),
        (
            "fb3655b9605f833c844a7903461d140e1726e7d5f84bc7e1c0f529bc365ad4fb",
            vec![",\n                }"],
        ),
        (
            "fb873f26eb837fe1a58ac2e1a18064cf1fbc85c2f7c1d378336f43c1c980fd3e",
            vec![",\n\n]"],
        ),
    ])
});

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=res");

    let cs_content_path = Path::new(CS_CONTENT_PATH);
    if cs_content_path.exists() {
        let cs_content = collect_cs_content(cs_content_path);
        let cs_content = cleaned_cs_content(cs_content);
        write_cs_content(cs_content);
    } else {
        println!(
            "cargo:warning=you need put CS content at {}",
            cs_content_path.display()
        )
    }

    let cs_images_path = Path::new(CS_IMAGES_PATH);
    if cs_images_path.exists() {
        let cs_images = collect_cs_images(cs_images_path);
        copy_cs_images(cs_images);
        copy_cs_images_as_general_icon(cs_images_path);
    } else {
        println!(
            "cargo:warning=you need put CS images at {}",
            cs_images_path.display()
        )
    }

    let boh_images_path = Path::new(BOH_IMAGES_PATH);
    if boh_images_path.exists() {
        copy_boh_images_as_general_icon(boh_images_path);
    } else {
        println!(
            "cargo:warning=you need put BoH images at {}",
            boh_images_path.display()
        )
    }

    let etc_static_files_path = Path::new(ETC_STATIC_PATH);
    let etc_static_files = collect_etc_static_files(etc_static_files_path);
    copy_etc_static_files(etc_static_files);
}

fn collect_cs_content(cs_content_path: &Path) -> Vec<PathBuf> {
    WalkDir::new(cs_content_path)
        .into_iter()
        .map(|entry| {
            entry
                .expect(&format!(
                    "a entry under '{}' is not available. Is that using by another program?",
                    cs_content_path.display()
                ))
                .into_path()
        })
        .filter(|entry| entry.is_file())
        .collect()
}

fn cleaned_cs_content(cs_content: Vec<PathBuf>) -> Vec<(PathBuf, String)> {
    cs_content
        .into_iter()
        .map(|path| {
            let content = fs::read(&path).expect(&format!(
                "'{}' is not available. Is that using by another program?",
                path.display()
            ));
            (path, content)
        })
        .map(|(path, mut content)| {
            if content.starts_with(&UTF8_BOM) {
                content.drain(0..3);
            }
            (path, content)
        })
        .map(|(path, content)| {
            let content = String::from_utf8(content)
                .expect(&format!("expect a UTF-8 or UTF-8 with BOM file"))
                .replace("\r\n", "\n");

            let mut buffer = Vec::new();
            let mut deserializer = Deserializer::from_str(&content);
            let mut serializer = Serializer::new(&mut buffer);

            let content =
            if serde_transcode::transcode(&mut deserializer, &mut serializer).is_ok() {
                buffer
            } else {
                let hash = {
                    let mut hasher = Sha256::new();
                    let bytes = fs::read(&path).expect(&format!(
                        "'{}' is not available. Is that using by another program?",
                        path.display()
                    ));
                    hasher.update(bytes);
                    hex::encode(hasher.finalize())
                };
                let patterns = HASH_PATTERNS
                    .get(hash.as_str())
                    .expect("unknown glitchy JSON file. Is the files up to date or patterns needed to update?");
                let content = {
                    let mut buffer = content;
                    for pattern in patterns {
                        let replacement = PATTERN_REPLACEMENT.get(pattern).expect("HASH_PATTERNS gives a bad pattern");
                        buffer = buffer.replace(pattern, replacement);
                    }
                    buffer
                };

                let mut buffer = Vec::new();
                let mut deserializer = Deserializer::from_str(&content);
                let mut serializer = Serializer::new(&mut buffer);
                serde_transcode::transcode(&mut deserializer, &mut serializer).expect(&format!("failed to fix JSON file"));

                buffer
            };

            (path, content)
        })
        .map(|(path, content)| (path, String::from_utf8(content).unwrap()))
        .collect()
}

fn write_cs_content(cs_content: Vec<(PathBuf, String)>) {
    for (path, content) in cs_content {
        let path = pathdiff::diff_paths(path, "res/assets/cs/content").unwrap();
        let path = OUT_CONTENT_CS_PATH.join(path);
        let parent = path.parent().unwrap();
        fs::create_dir_all(parent)
            .expect(&format!("failed to create folders '{}'", parent.display()));
        fs::write(&path, content).expect(&format!("failed to write file '{}'", path.display()));
    }
}

fn collect_cs_images(cs_images_path: &Path) -> Vec<PathBuf> {
    WalkDir::new(cs_images_path)
        .into_iter()
        .map(|entry| {
            entry
                .expect(&format!(
                    "a entry under '{}' is not available. Is that using by another program?",
                    cs_images_path.display()
                ))
                .into_path()
        })
        .filter(|entry| entry.is_file())
        .filter(|entry| {
            entry
                .extension()
                .is_some_and(|extention| extention == "png")
        })
        .collect()
}

fn copy_cs_images(cs_images: Vec<PathBuf>) {
    for source in cs_images {
        let relative_path = pathdiff::diff_paths(&source, "res/assets/cs/images").unwrap();
        let destination = OUT_STATIC_IMAGES_CS_PATH.join(relative_path);
        let parent = destination.parent().unwrap();
        fs::create_dir_all(parent)
            .expect(&format!("failed to create folders '{}'", parent.display()));
        fs::copy(&source, &destination)
            .expect(&format!("failed to copy file '{}'", source.display()));
    }
}

fn collect_etc_static_files(etc_static_files_path: &Path) -> Vec<PathBuf> {
    WalkDir::new(etc_static_files_path)
        .into_iter()
        .map(|entry| {
            entry
                .expect(&format!(
                    "a entry under '{}' is not available. Is that using by another program?",
                    etc_static_files_path.display()
                ))
                .into_path()
        })
        .filter(|entry| entry.is_file())
        .collect()
}

fn copy_etc_static_files(etc_static_files: Vec<PathBuf>) {
    for source in etc_static_files {
        let relative_path = pathdiff::diff_paths(&source, "res/static").unwrap();
        let destination = OUT_STATIC_PATH.join(relative_path);
        let parent = destination.parent().unwrap();
        fs::create_dir_all(parent)
            .expect(&format!("failed to create folders '{}'", parent.display()));
        fs::copy(&source, &destination)
            .expect(&format!("failed to copy file '{}'", source.display()));
    }
}

fn copy_cs_images_as_general_icon(cs_images_path: &Path) {
    let favicon = PathBuf::new()
        .join(cs_images_path)
        .join("aspects")
        .join("secrethistories.png");
    let destination = OUT_STATIC_IMAGES_PATH.join("favicon.png");
    fs::copy(&favicon, &destination).unwrap();

    let favicon = image::open(favicon).unwrap();
    let destination = OUT_STATIC_IMAGES_PATH.join("favicon.ico");
    favicon
        .save_with_format(destination, ImageFormat::Ico)
        .unwrap();
}

fn copy_boh_images_as_general_icon(boh_images_path: &Path) {
    let icon = PathBuf::new()
        .join(boh_images_path)
        .join("elements")
        .join("numen.worl.png");
    let destination = OUT_STATIC_IMAGES_PATH.join("icon.png");
    fs::copy(icon, destination).unwrap();

    let codex = PathBuf::new()
        .join(boh_images_path)
        .join("aspects")
        .join("codex.png");
    let destination = OUT_STATIC_IMAGES_PATH.join("codex.png");
    fs::copy(codex, destination).unwrap();
}
