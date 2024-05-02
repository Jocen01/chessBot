pub const HORSE_BIT_MOVES: [u64;64] = [132096,329728,659712,1319424,2638848,5277696,10489856,4202496,33816580,84410376,168886289,337772578,675545156,1351090312,2685403152,1075839008,8657044482,21609056261,43234889994,86469779988,172939559976,345879119952,687463207072,275414786112,2216203387392,5531918402816,11068131838464,22136263676928,44272527353856,88545054707712,175990581010432,70506185244672,567348067172352,1416171111120896,2833441750646784,5666883501293568,11333767002587136,22667534005174272,45053588738670592,18049583422636032,145241105196122112,362539804446949376,725361088165576704,1450722176331153408,2901444352662306816,5802888705324613632,11533718717099671552,4620693356194824192,288234782788157440,576469569871282176,1224997833292120064,2449995666584240128,4899991333168480256,9799982666336960512,1152939783987658752,2305878468463689728,1128098930098176,2257297371824128,4796069720358912,9592139440717824,19184278881435648,38368557762871296,4679521487814656,9077567998918656];
pub const KINGS_BIT_MOVES: [u64;64] = [770,1797,3594,7188,14376,28752,57504,49216,197123,460039,920078,1840156,3680312,7360624,14721248,12599488,50463488,117769984,235539968,471079936,942159872,1884319744,3768639488,3225468928,12918652928,30149115904,60298231808,120596463616,241192927232,482385854464,964771708928,825720045568,3307175149568,7718173671424,15436347342848,30872694685696,61745389371392,123490778742784,246981557485568,211384331665408,846636838289408,1975852459884544,3951704919769088,7903409839538176,15806819679076352,31613639358152704,63227278716305408,54114388906344448,216739030602088448,505818229730443264,1011636459460886528,2023272918921773056,4046545837843546112,8093091675687092224,16186183351374184448,13853283560024178688,144959613005987840,362258295026614272,724516590053228544,1449033180106457088,2898066360212914176,5796132720425828352,11592265440851656704,4665729213955833856];
// pub const WHITE_PAWN_BIT_MOVES: [u64;64] = [768,1792,3584,7168,14336,28672,57344,49152,16973824,34013184,68026368,136052736,272105472,544210944,1088421888,2160066560,50331648,117440512,234881024,469762048,939524096,1879048192,3758096384,3221225472,12884901888,30064771072,60129542144,120259084288,240518168576,481036337152,962072674304,824633720832,3298534883328,7696581394432,15393162788864,30786325577728,61572651155456,123145302310912,246290604621824,211106232532992,844424930131968,1970324836974592,3940649673949184,7881299347898368,15762598695796736,31525197391593472,63050394783186944,54043195528445952,216172782113783808,504403158265495552,1008806316530991104,2017612633061982208,4035225266123964416,8070450532247928832,16140901064495857664,13835058055282163712,0,0,0,0,0,0,0,0];
pub const ZOBRIST_WHITE_PAWN: [u64; 64] = [6445487328462042386,9816340805328526479,14276037255240723858,9956733384956380361,14356711736173799329,5233932826836581804,162098087903251788,2382719427611251566,18260522736096929330,18229348070592149400,15358686575854799521,7620041910258410308,578053582090587809,2865029909199473904,3884666383624410360,11538994289337366521,11769176063606318795,17531918161079043613,101112962882070795,11972713318910146812,1855023726551084955,5420032434227273556,16894765470757003602,3498610325907181649,17548355022144124459,2757060866482314177,13476715761226472055,9740161913176161471,4176240320695215606,2332476496004462485,15609972682217493578,8339458211199840193,14274430118011961223,8928141053678082330,16403131681633602763,6534064924682863571,8322729449239912463,2916373428911708567,11641653441524018692,6356389727710354776,13794596582905221011,6432373113411367189,227086125643333648,11548360651811384376,15238187363652176844,15216162087377636556,396632450294256731,9220357413199943354,812278009316018597,10094198834684321733,15665219210187679171,3826703023725598099,15341906537952684799,426189172709877413,10094512602418575346,16149701301675310426,14650389334925358859,9361560137516153853,15884465633143994913,5095384757331580603,17294635708070103093,6631731388691083549,517111071914175058,5215886274222583884];
pub const ZOBRIST_WHITE_KNIGHT: [u64; 64] = [4676950732724562788,12900889875545356275,4188905412191169387,16370015149485743876,2053754414252303670,13838005098444124820,4231399401723348429,15739403626640445542,10101582614696124603,6535706725201138216,16039608227282638856,14377413572733041151,14975963619652079172,4221467893222292698,17326818864978698215,5406950511997023250,7577700282078373476,1611463315126805277,1828952478585070253,5926399837333519511,11150548489592345642,18419633457820074441,15531125320704132825,7378708043307197665,4261652661101140273,1445436714868920892,2194917670223616954,6412290088651331522,1678395790941188011,9955110391891820654,3002895660729559737,3876438365782305023,15343724799444853801,11259941458638056729,8887604691847274309,1701990588255970642,3767217407086578028,17085924543467307166,13119439135683835807,15249583095183397563,5287901915960928426,16046584780595701705,953827401384425751,17582803652658030614,14298593854316993069,18135099981719029368,14945213791583857311,12984733893834910505,5224651178175622090,11076414174678324064,12743535578879118967,5210720790712791647,763520517566880266,8254171453996605474,12410507574780774652,10847180073070907815,15361560452660234976,16758023332992244563,3301296646030330340,7185735127230104372,17411173307446160392,1123159489246262328,11907634403516555581,6689047123333275129];
pub const ZOBRIST_WHITE_BISHOP: [u64; 64] = [15040685037182610195,8751232703495722868,4486198616264935852,9912747374320136425,3133844653820236730,9476121996061586079,12262461030918753716,3391804270070874607,1041197693479746672,3601402769651921845,4719966885359263863,9617961664430556086,1754793510788732713,13639292729320913516,13145110728503450755,12084156847310887707,9648104346047208549,10217302975015882361,15735858764061113934,9327906645592441757,9499738297899650776,17863369338820076787,18328745800675061401,14181947353934228251,9873177778730183955,16974521535180362607,8575526011064405356,17063707572605081702,6447743265344421580,16562128750324856350,2070986129287105275,1618396890984212380,14980755934912256805,15903133371060057148,3417328847766729243,14277526536160421358,6066919287758696893,14513081124098850899,4436375607651812365,1067001822171776937,17109353537368612922,10001160682408548278,8441429441744601631,17484291520508733250,8487890334352511560,3463981995470319002,17234896107136245118,9554468343201591432,11684015879922106603,1136071759999606894,6033086363580310453,15448171624010032155,5959097742720027668,16969835850127516720,7789104480295224576,8276838855515510514,13911398692362552809,16025339787121929543,10840167171603968560,1370864255064292659,17834816316273090098,8887702586262849706,18339929851503082086,12368414606712352461];
pub const ZOBRIST_WHITE_ROOK: [u64; 64] = [16344178861511476198,8949315360391405377,17328039345572309173,8615333056258294825,16429687426642074620,16861884240419497543,17313182791018373744,12277596749771607452,49921652312893804,18083431545476028167,6047306765234637270,11410636983829142409,5656387894864644450,5671057917343525644,9095722753689411562,15410605095921716943,9468252230070761580,11790170765741196758,7251245233266373813,7690961398473064520,376653421657030977,4356289048187370502,2065686466076444410,5006057000354185663,4641490570937711407,15811859347591061105,15412538087048759478,3973901039283043840,12908309058003788574,6651932560526657925,1269152058149006841,1366593577175384155,9244822479424953143,9344503651737592511,3477605779784410469,8460764306177691972,4851933965367241666,17718278227316194997,16275716468280545460,8050721642461216174,1987205301062292846,17283026709885021321,9901110794643953285,13639103056634960213,16753146814180636804,10314210859740637343,15815968832399290451,2296988452865584471,5219996002854755496,6959626128313918842,1297916695979361492,13064589819603909950,6033540282365077121,4687777955641849885,13362721226597688447,4576850288408119260,15403965008327904381,3377007263223575635,5456148075310176423,9870582294190489092,16570030092978611442,10400166802168804663,16545554202774625839,4441010541244289270];
pub const ZOBRIST_WHITE_QUEEN: [u64; 64] = [2922421232583821636,11968953979086771629,17223947520705523147,5743185391185209723,13455352105389613384,12964716740267986897,10832768457731247792,4427875116126576389,12635018036902665574,6658016128079711171,6810415084325261573,15069760620872120363,13765986664217200614,4536950571190418165,3497534269450658544,17560521927847515880,18009756682607038949,12414144665983030377,7550169662154093016,6678473876971933708,13103812842514076426,15084900861771262122,18180147032899104613,11695468761195775278,61165643059282143,3406599899014319569,313573971855192921,695269005128661051,5187265937828009594,18360261859647478591,1429849731062008363,6665143695251394461,7498948616957915488,17939509824822785682,3545276933256644092,11086872053454432458,10152566352087136166,15694687324943172044,3097081403629722102,13734093502858044818,15376010313474960686,9378134172377556970,2267137892060105780,7131206073364674529,9737644678936546905,5242051831451455920,16183733415034605880,14112506717567870808,3164035512002787372,6796225691892375846,18123153496127414276,10037329809152850124,13816162309906510168,15267397820707947618,6121475846576944055,15192036955670054868,14796653474536440540,18118184158491582925,10698307871369165089,9131102585988432944,8188324224322618165,4810089612523785473,14342837914757230909,5178199750020086551];
pub const ZOBRIST_WHITE_KING: [u64; 64] = [9704436776008929152,1752355896774313797,12152426974768451893,3444793076584363006,14839415803258206535,3361863395933641741,11529337078934166526,17283671358433038581,13508643452318635582,4145027720355219826,14737410984923494036,6887908166418716778,13267257614071490261,6004847302005524072,13135673158760292957,10386928930654641494,6662887267914330007,12066345970981673682,4345727316466093200,11139326766587020923,15144703182867805539,4343010836784886080,4772932464756556778,4437009192275512069,14083368942076064863,7272672172304599684,3996190086867621561,12887284420369036791,4687105893545960919,1706438336499544518,2209049320118952926,8088046162960488140,10493695778206644103,11406808561135143836,7362691635262669870,17646369941957224971,6005403670468570479,15134458280103618235,11354480449403460200,12538423002940448852,7147969402782734632,7737595327383715551,17414660606161731492,2327057064548035839,16349974251796702762,13813012504887521158,2946051742732242236,11733630187044338734,15231199376384095374,413082133232116425,11585099165867585118,11335340791544184380,10208408672839008190,4930961915122722939,5480806250987211027,13074811268210682393,1433983856076328578,11904486398530378605,8935120100891506909,18238959268702746751,125626108672557270,12699363013555184192,8836641430168187942,2123016208199443314];
pub const ZOBRIST_BLACK_PAWN: [u64; 64] = [11353627601992937319,4675812802480207805,7445039320897740169,11137593704460550172,15038392677931049487,7714227260498653977,11409278359789030453,6053235297963048759,16425870104029889429,13780214783659075062,1075533596419592961,1918017775433201003,8775853111714681320,1338358918771306489,8259421671651883660,2458496190133530696,17596576360582221937,12369787384522655923,17273383828943964178,4227790670546790117,2678813953822262425,11633419883558902046,12383245372211322938,44329718933787299,1378402317075154189,17109592467001424031,11263390327935475541,5183114021677258373,3879344864426777303,15933059364369071175,7510488635581781461,7760010390838279064,4260380343209992025,11317909927766307687,14761169182487948302,3872461028571892024,14122523834994257860,12442346949040807372,9375792602065787132,9569422989794502056,13400588394514140024,8176959828295207038,7007125648130958085,16715042734707048781,6488916777808698153,16284539424207640535,2348004681794720211,12813925422140965432,5772088687534267020,6320518278917055448,13461330018773511103,12969529263752196076,12906251121970683525,6229371974400990992,9690912729836647850,3196347176478446898,14862711284591139326,7147370266382657602,8735625569458286345,17071188947377074401,13995459964201674280,1735369173173009608,10591890757647869974,3981191858076750824];
pub const ZOBRIST_BLACK_KNIGHT: [u64; 64] = [13909640885596139085,17825888276357450846,8346364615013357777,8530351055694756654,12559500759818545475,11193545411151740738,7857009035440693924,4579400615074608597,1637032538802630628,10734113488575727949,11888361261041426000,16295108652188079710,812799691079784985,2574441552303227000,8442028232170131782,16885796295448600806,3229802955431764479,15641316787285348281,2403844945530713752,2115651313290286491,3701593595195736447,7611388744534840027,10297327013364363059,12115292756533242893,506492032843989942,2702259637743398625,11345754111948541943,2898869683317120834,12672529428025031646,3901280268278133435,7958194587133219327,2735300286282762128,15525310342804436290,7572760271461065689,10565922331788992653,10743222103513410328,18145898694681464529,11758886835992116024,10582672130543258588,2925555759628989962,1918466052309885950,15927262691931370997,10061726489243024144,8592454836381074012,10606287033503079479,1999019062145913776,5971590146390351156,1107229992947496171,4681688617946334859,2262473458312333146,4317643344725867604,3684015725894595326,16208126301394909982,15090036461945083801,16547275597025505835,10419421792595849461,9845022781346796595,3201738427194949972,1930840162582775306,16988567126559777781,6842162218171171253,792129001332208388,11332932509743733099,4643584851975106489];
pub const ZOBRIST_BLACK_BISHOP: [u64; 64] = [11229732817612822575,6574688135288471664,5176494743518960090,7749327457181168061,15718563715000891109,13996923738012864089,5459203512918809715,8265613339493849326,39004773559749753,5501023880237145868,9521766345553604351,17982714783963096294,5132563426349500889,175032296821276965,12447212516275308384,5890662243101580562,14929460284951687292,4299532056003047797,5983490256101622498,5434997416098281349,13821604621138750338,14871304950427481302,5945977588078892741,6449520036518144121,14986472261984446330,1779196436575767941,7688965872397200690,15461812778093743966,18413184066962492041,1199564710256700802,3021950265369556700,9622689661456178475,4576293679125841389,11418483025227425761,228604507592843122,12570097789264944210,12466136892044695105,11604388482698005567,2328927030681886445,17577376342844497630,983977915762136870,7595046360012822590,2364116015773188553,17802708102947476797,10247753139837398880,902960415971653118,9958590747805826369,6864566896912900944,1968448401266488379,4671121526141958810,10081205256320795785,6376870540323076289,14343267436996887502,10706188637445592587,144241822923397418,11329592902358001779,15010481053334932803,7400953352203379058,6391611457290315216,12701599928996195349,15008124928423326803,11774997504292037497,4215734967059375917,8945593503764062041];
pub const ZOBRIST_BLACK_ROOK: [u64; 64] = [11830013220183959468,1164228601886663599,14370829569893263003,1930467261609832995,13676089293019429760,5999972304301928826,884790853007748954,1214512948094515904,430440139426103677,17020452409794242040,5642573235394982670,14087921535068926069,9134027496364709076,6021487179742448217,809339800074152636,17959332054519475554,4994844189993300118,14096384254112287061,8054744341899085546,16239414248121800323,14684235555514737970,5859017153080914218,15929318624579972381,15492612316129407103,12791879233979572021,1188041480482021452,2943744479194875572,13571435140939366883,6247790440459689743,7096143035921171475,15825352587553539079,12549208572631065846,2946848771585835632,8853245099064056683,2935944307230371192,5538225686071443123,15728400779588045726,11197987521766851693,4803428923775910121,16474733422351432876,8955389575182449186,6174641732731582494,9765359353978470995,5976660187499304854,16391435822735418492,17253188408686316633,1989869704415640673,5535759236155096662,5527281646825202126,1598147542846021487,7824865108356348571,5445349657917129497,10625216369573025433,11288310878565799131,8930014320984235639,7016863510267459233,525195844955956020,1657197203427186235,357820451923548569,646409232982243734,4459576367201517982,6070598893589553763,6866303199774797709,10365416684716135558];
pub const ZOBRIST_BLACK_QUEEN: [u64; 64] = [7867472115090344627,16754657208974639758,18054487933882033525,4148799799228142476,10524833494754652661,17806027008137493740,17208536698036303726,59061871162850133,15039149504693696160,4098116402427896807,9016069296659430864,4960479531156116624,14511945333342197073,1033844783825042528,9954972377507242622,3519363910106863176,2055777073833325539,10794136983970011435,17314336634780081844,11915505835658390556,3068781002453540709,6376786907502507035,9037140054374563993,10748620347628767216,3965853755098813372,9391076664244023468,11742744889835904748,5333010302968259383,12840205061639117494,9228247494373630703,13179100015743911662,6959790791692212566,10294305326509195687,14511343647579477676,9331718409799462162,17379856622005876635,10997339335587424011,7835489756522154942,2087825678696291310,17258357033227998300,3520883098114472412,17578178952047094352,5974780507585343447,13522504576458507266,9725970945683847893,12807043896524319427,5007673820428445687,12217499868504452040,15156633468822649033,17763857494451637268,11907988472239389649,10565277405062829912,15094837912540183452,7977298012858595771,11052330947087951993,10922340471847130807,123880336341503800,6639947688645028258,12090498126018604938,11907992639230288201,4006558718899710514,911944460454249291,14510847007675947855,10752992404591075964];
pub const ZOBRIST_BLACK_KING: [u64; 64] = [741076008418966670,16328507174064163837,1032868708343941825,14276958848659775971,17047540293913078090,1428354548236298741,12287683029260292914,9898592453221038442,5668329142215531406,13777100325580724426,14873463314147752538,2529785323108363566,4970004863752013375,5140341108809410608,15386224518319792744,6367132614535829501,1962175440422006436,18000045952262524561,5950872885970022323,3086801078710986837,8799121201376294157,14391365018804072444,7103639924459756646,6969645909482380148,14604526433475323746,17380966943543636034,18191747398293774776,14416810525187864914,6587046974742645904,12049359722551804295,15644912378334194088,16624311851151735532,9020857077532424949,10801741452366645042,7704733693622957754,9911959762852724631,16021170102636868543,7383421831085836194,7789171361007940882,10952232725882675958,5988096928164418768,7803881898188289285,12858591869647658419,4564384895381143166,15209640880466709105,13336983777982688729,3369338576281936971,14652541724432865971,14788632152010371907,14133573054907114206,2696453687835859640,8520389161923829609,12999394182495178770,5540093061598135601,12977490994448419095,4670502030087813394,3335419388429659062,1719324284342057429,2476872097025980841,8185903223328207054,11183439937783390862,16801412261039721356,5439904004964595883,7269284751921246986];
pub const ZOBRIST_CASTLE_RIGHTS: [u64; 16] = [8604494402185362885,4953029046216010287,16159619028766221045,7192728816492759635,8842683537334884211,14893315272591892536,8036895720380320278,9497428325562413949,15826551101066735117,5900921102123293292,10149181328674082917,2764839986575300064,17818137433686728461,15138577488830048114,13926163082112329880,11231407108359624811];
pub const ZOBRIST_TURN_COLOR: [u64; 2] = [15008460703448942654, 14151944575884939266];
pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 300;
pub const BISHOP_VALUE: i32 = 320;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;