lazy_static! {
    pub static ref SOLSTICES: Vec<i64> = {
        vec![
            14845368,
            30695734,
            46401583,
            62252626,
            77958380,
            93809567,
            109515645,
            125366856,
            141071866,
            156923749,
            172628795,
            188480726,
            204186262,
            220037700,
            235743235,
            251594581,
            267300583,
            283152051,
            298857379,
            314708978,
            330414430,
            346265759,
            361971890,
            377823025,
            393528189,
            409379880,
            425084930,
            440936987,
            456642144,
            472494162,
            488198658,
            504050855,
            519755407,
            535608121,
            551311854,
            567164746,
            582868602,
            598721267,
            614425991,
            630278513,
            645982376,
            661835211,
            677539136,
            693392011,
            709096457,
            724948987,
            740653193,
            756505540,
            772210062,
            788062954,
            803766871,
            819620197,
            835323832,
            851177145,
            866881203,
            882734815,
            898437762,
            914291778,
            929994555,
            945848620,
            961552071,
            977405837,
            993109074,
            1008962481,
            1024665874,
            1040519653,
            1056222637,
            1072076618,
            1087779422,
            1103632888,
            1119336376,
            1135190087,
            1150892761,
            1166746916,
            1182449193,
            1198303660,
            1214006371,
            1229861016,
            1245563140,
            1261417598,
            1277119714,
            1292974698,
            1308676600,
            1324531793,
            1340233737,
            1356088288,
            1371791047,
            1387645853,
            1403347883,
            1419202972,
            1434904683,
            1450759668,
            1466462060,
            1482317042,
            1498019057,
            1513873668,
            1529575643,
            1545430954,
            1561132461,
            1576988355,
            1592689426,
            1608544929,
            1624246334,
            1640102345,
            1655802835,
            1671659278,
            1687359473,
            1703215627,
            1718916661,
            1734772818,
            1750473736,
            1766329368,
            1782030268,
            1797886196,
            1813587047,
            1829443310,
            1845144116,
            1860999560,
            1876700893,
            1892556825,
            1908257474,
            1924114154,
            1939814221,
            1955670908,
            1971371318,
            1987228531,
            2002928459,
        ]
    };
    pub static ref NEW_MOONS: Vec<i64> = {
        vec![
            592528,
            3136370,
            5679747,
            8222961,
            10767054,
            13314072,
            15866266,
            18424686,
            20988072,
            23553090,
            26116074,
            28674850,
            31228959,
            33778507,
            36323313,
            38863409,
            41400100,
            43936306,
            46475838,
            49022097,
            51576799,
            54139332,
            56707136,
            59276747,
            61844566,
            64407143,
            66961726,
            69507281,
            72045052,
            74578079,
            77110195,
            79645124,
            82185948,
            84734905,
            87293276,
            89860877,
            92435042,
            95010149,
            97579369,
            100138045,
            102685503,
            105224092,
            107757254,
            110288323,
            112820326,
            115356317,
            117899630,
            120453389,
            123018905,
            125593621,
            128170913,
            130743224,
            133305837,
            135857788,
            138400455,
            140936138,
            143467588,
            145998092,
            148531513,
            151071886,
            153622392,
            156183884,
            158753976,
            161327811,
            163900045,
            166466338,
            169023892,
            171571745,
            174111006,
            176644640,
            179176720,
            181711395,
            184251885,
            186799802,
            189355202,
            191917209,
            194484285,
            197053675,
            199621168,
            202182405,
            204734995,
            207279522,
            209818832,
            212356491,
            214895377,
            217437038,
            219982066,
            222531065,
            225085011,
            227644357,
            230207725,
            232771882,
            235333361,
            237890187,
            240442258,
            242990565,
            245536241,
            248080162,
            250623164,
            253166382,
            255711239,
            258258972,
            260810109,
            263364402,
            265921286,
            268480217,
            271040441,
            273600532,
            276158438,
            278712392,
            281261967,
            283808153,
            286352372,
            288895513,
            291437969,
            293980484,
            296524822,
            299073469,
            301628436,
            304189825,
            306755195,
            309320583,
            311882618,
            314439802,
            316991963,
            319539053,
            322080953,
            324618374,
            327153611,
            329690297,
            332232338,
            334782557,
            337341611,
            339907778,
            342477752,
            345047703,
            347613840,
            350172825,
            352722659,
            355263566,
            357797950,
            360329501,
            362862184,
            365399516,
            367944204,
            370498032,
            373061597,
            375633501,
            378209400,
            380782557,
            383346786,
            385899448,
            388441720,
            390976818,
            393508304,
            396039397,
            398573091,
            401112545,
            403661046,
            406220994,
            408791884,
            411368867,
            413944309,
            416511805,
            419068698,
            421615504,
            424154250,
            426687508,
            429218274,
            431750097,
            434286938,
            436832467,
            439388751,
            441954939,
            444527185,
            447100251,
            449669372,
            452231117,
            454783669,
            457327106,
            459863473,
            462396329,
            464929837,
            467467688,
            470012200,
            472563996,
            475122495,
            477686560,
            480254320,
            482822516,
            485386866,
            487943875,
            490492579,
            493034729,
            495573593,
            498112397,
            500653220,
            503196866,
            505743701,
            508294518,
            510850292,
            513410887,
            515974176,
            518536823,
            521096088,
            523650943,
            526201821,
            528749685,
            531295328,
            533839357,
            536382587,
            538926270,
            541471837,
            544020332,
            546572062,
            549126800,
            551684208,
            554243849,
            556804715,
            559364892,
            561922073,
            564474774,
            567023116,
            569568336,
            572111655,
            574653739,
            577195200,
            579737432,
            582282827,
            584833980,
            587392257,
            589956548,
            592523320,
            595088385,
            597648959,
            600204133,
            602753817,
            605297920,
            607836762,
            610371982,
            612906765,
            615445141,
            617990734,
            620545470,
            623108811,
            625678024,
            628249241,
            630818375,
            633381602,
            635936064,
            638480900,
            641017647,
            643549627,
            646080874,
            648615256,
            651155939,
            653705181,
            656264202,
            658832675,
            661407700,
            663983379,
            666552702,
            669111030,
            671657856,
            674195748,
            676728354,
            679259164,
            681791261,
            684327648,
            686871528,
            689425860,
            691991778,
            694566575,
            697143579,
            699715329,
            702277286,
            704828666,
            707370994,
            709906680,
            712438509,
            714969711,
            717504008,
            720045225,
            722596278,
            725157771,
            727727212,
            730299905,
            732870864,
            735436137,
            737993190,
            740541146,
            743081041,
            745615695,
            748149016,
            750684956,
            753226460,
            755774814,
            758329821,
            760890593,
            763455875,
            766023424,
            768589593,
            771150386,
            773703437,
            776249107,
            778789964,
            781329307,
            783869729,
            786412443,
            788957738,
            791506063,
            794058466,
            796615716,
            799176978,
            801739626,
            804300595,
            806857975,
            809411456,
            811961683,
            814509372,
            817054968,
            819598945,
            822142227,
            824686207,
            827232283,
            829781325,
            832333570,
            834888946,
            837447289,
            840008028,
            842569636,
            845129666,
            847685769,
            850236978,
            852783942,
            855327975,
            857870074,
            860410913,
            862951591,
            865494212,
            868041584,
            870596033,
            873157893,
            875724691,
            878292064,
            880856041,
            883414592,
            885967250,
            888513955,
            891054818,
            893590882,
            896124733,
            898660216,
            901201425,
            903751384,
            906310889,
            908878164,
            911449606,
            914020941,
            916587965,
            919147122,
            921696477,
            924236507,
            926769900,
            929300571,
            931832636,
            934369709,
            936914535,
            939468865,
            942033182,
            944605898,
            947182419,
            949755796,
            952319803,
            954871920,
            957413523,
            959948036,
            962479194,
            965010308,
            967544357,
            970084377,
            972633481,
            975193878,
            977764897,
            980341610,
            982916466,
            985483260,
            988039536,
            990585961,
            993124664,
            995658263,
            998189715,
            1000722441,
            1003260198,
            1005806399,
            1008362843,
            1010928516,
            1013499652,
            1016071351,
            1018639267,
            1021200304,
            1023752790,
            1026296760,
            1028834110,
            1031368217,
            1033903053,
            1036442066,
            1038987262,
            1041538970,
            1044096502,
            1046658897,
            1049224721,
            1051791286,
            1054354791,
            1056911919,
            1059461562,
            1062005179,
            1064545750,
            1067086218,
            1069628337,
            1072172580,
            1074719093,
            1077268660,
            1079822479,
            1082380871,
            1084942315,
            1087504005,
            1090063424,
            1092619431,
            1095172141,
            1097722095,
            1100269631,
            1102814942,
            1105358568,
            1107901681,
            1110445820,
            1112992320,
            1115541926,
            1118094905,
            1120651348,
            1123211083,
            1125773123,
            1128335272,
            1130894676,
            1133449256,
            1135998704,
            1138544078,
            1141086646,
            1143627315,
            1146167033,
            1148707536,
            1151251515,
            1153801853,
            1156360185,
            1158925503,
            1161494044,
            1164061081,
            1166623247,
            1169179241,
            1171728858,
            1174272154,
            1176809761,
            1179343634,
            1181877188,
            1184414626,
            1186959749,
            1189514653,
            1192078840,
            1194649385,
            1197222025,
            1199792230,
            1202355871,
            1204910054,
            1207454118,
            1209989893,
            1212520953,
            1215051514,
            1217585552,
            1220126282,
            1222675935,
            1225235631,
            1227804875,
            1230380543,
            1232956518,
            1235525705,
            1238083556,
            1240629753,
            1243167059,
            1245699297,
            1248230075,
            1250762495,
            1253299459,
            1255843985,
            1258398824,
            1260964926,
            1263539483,
            1266115879,
            1268686866,
            1271248135,
            1273799063,
            1276341274,
            1278877226,
            1281409688,
            1283941789,
            1286477068,
            1289019106,
            1291570542,
            1294131757,
            1296700240,
            1299271552,
            1301841139,
            1304405441,
            1306962155,
            1309510434,
            1312051187,
            1314587044,
            1317121720,
            1319658947,
            1322201381,
            1324749983,
            1327304357,
            1329863675,
            1332427025,
            1334992703,
            1337557620,
            1340118125,
            1342671840,
            1345218864,
            1347761437,
            1350302550,
            1352844480,
            1355388097,
            1357933417,
            1360480806,
            1363031460,
            1365586517,
            1368145702,
            1370706979,
            1373267656,
            1375825840,
            1378380967,
            1380933271,
            1383482997,
            1386030142,
            1388574850,
            1391117912,
            1393660779,
            1396205080,
            1398752060,
            1401302412,
            1403856507,
            1406414504,
            1408975964,
            1411539225,
            1414101399,
            1416659536,
            1419212151,
            1421759621,
            1424303235,
            1426844170,
            1429383410,
            1431922393,
            1434463519,
            1437009859,
            1439564002,
            1442126475,
            1444694743,
            1447264029,
            1449829765,
            1452389432,
            1454942335,
            1457488469,
            1460028220,
            1462562970,
            1465095574,
            1467630059,
            1470170671,
            1472720585,
            1475280682,
            1477849091,
            1480421894,
            1482994391,
            1485562021,
            1488121102,
            1490669833,
            1493208968,
            1495741467,
            1498271441,
            1500803133,
            1503340209,
            1505885391,
            1508440323,
            1511005328,
            1513578625,
            1516155434,
            1518728712,
            1521292294,
            1523843827,
            1526384866,
            1528918993,
            1531450071,
            1533981463,
            1536516087,
            1539056810,
            1541606523,
            1544167221,
            1546738091,
            1549314214,
            1551888238,
            1554454228,
            1557009928,
            1559556115,
            1562094970,
            1564629113,
            1567161427,
            1569695180,
            1572233906,
            1574780734,
            1577337186,
            1579902118,
            1582471919,
            1585042091,
            1587608749,
            1590169129,
            1592721684,
            1595266374,
            1597804897,
            1600340410,
            1602876660,
            1605416829,
            1607962593,
            1610514008,
            1613070338,
            1615630866,
            1618194647,
            1620759584,
            1623322355,
            1625879793,
            1628430604,
            1630975902,
            1633518320,
            1636060473,
            1638603779,
            1641148407,
            1643694358,
            1646242483,
            1648794261,
            1651350482,
            1653910213,
            1656471132,
            1659030897,
            1661588222,
            1664142869,
            1666694918,
            1669244230,
            1671790609,
            1674334391,
            1676876745,
            1679419383,
            1681963947,
            1684511592,
            1687063023,
            1689618704,
            1692178684,
            1694741982,
            1697306104,
            1699867640,
            1702423917,
            1704974239,
            1707519545,
            1710061220,
            1712600446,
            1715138510,
            1717677458,
            1720220237,
            1722769976,
            1725328528,
            1727894951,
            1730465223,
            1733034080,
            1735597602,
            1738154152,
            1740703483,
            1743245864,
            1745782263,
            1748314934,
            1750847489,
            1753384264,
            1755929185,
            1758484441,
            1761049505,
            1763621230,
            1766194994,
            1768765911,
            1771329661,
            1773883401,
            1776426701,
            1778961655,
            1781492042,
            1784022208,
            1786556196,
            1789097212,
            1791647398,
            1794207720,
            1796777503,
            1799353455,
            1801929359,
            1804498161,
            1807055462,
            1809601109,
            1812138012,
            1814670116,
            1817201105,
            1819734062,
            1822271756,
            1824816985,
            1827372257,
            1829938331,
            1832512342,
            1835087836,
            1837657870,
            1840218407,
            1842768968,
            1845311242,
            1847847692,
            1850381021,
            1852914215,
            1855450599,
            1857993472,
            1860545169,
            1863105861,
            1865673081,
            1868242745,
            1870810800,
            1873374117,
            1875930621,
            1878479452,
            1881021336,
            1883558652,
            1886094862,
            1888633436,
            1891176717,
            1893725362,
            1896278840,
            1898836469,
            1901397741,
            1903961517,
            1906525268,
            1909085656,
            1911640247,
            1914188829,
            1916733268,
            1919276207,
            1921819577,
            1924363921,
            1926909048,
            1929455319,
            1932004132,
            1934557012,
            1937114219,
            1939674268,
            1942234801,
            1944793924,
            1947350799,
            1949905230,
            1952456964,
            1955005535,
            1957550789,
            1960093442,
            1962635066,
            1965177556,
            1967722530,
            1970271112,
            1972824076,
            1975381877,
            1977944182,
            1980509167,
            1983073483,
            1985633558,
            1988187407,
            1990735177,
            1993278197,
            1995817883,
            1998355556,
            2000892977,
        ]
    };
}
