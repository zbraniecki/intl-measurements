#include <unicode/locid.h>
#include <unicode/utypes.h>
#include <unicode/bytestream.h>
#include <stdio.h>
#include <chrono>
#include <vector>
#include <cstring>

using namespace std;

static void show(void);

void show(void)
{
  vector<string> ids;
  vector<icu_64::Locale> locales;


  ids.push_back("en-US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("sl");
  ids.push_back("sk");
  ids.push_back("ur");
  ids.push_back("pa-IN");
  ids.push_back("mai");
  ids.push_back("kab");
  ids.push_back("uz");
  ids.push_back("pl");
  ids.push_back("vi");
  ids.push_back("he");
  ids.push_back("bn-BD");
  ids.push_back("ms");
  ids.push_back("ne-NP");
  ids.push_back("km");
  ids.push_back("be");
  ids.push_back("da");
  ids.push_back("sv-SE");
  ids.push_back("gn");
  ids.push_back("kk");
  ids.push_back("mr");
  ids.push_back("mn");
  ids.push_back("pt-BR");
  ids.push_back("ja");
  ids.push_back("el");
  ids.push_back("it");
  ids.push_back("ca");
  ids.push_back("zh-TW");
  ids.push_back("nb-NO");
  ids.push_back("cs");
  ids.push_back("ia");
  ids.push_back("te");
  ids.push_back("pt-PT");
  ids.push_back("ach");
  ids.push_back("ru");
  ids.push_back("hi-IN");
  ids.push_back("tl");
  ids.push_back("ro");
  ids.push_back("hsb");
  ids.push_back("zh-CN");
  ids.push_back("cak");
  ids.push_back("hy-AM");
  ids.push_back("gu-IN");
  ids.push_back("su");
  ids.push_back("uk");
  ids.push_back("sr");
  ids.push_back("si");
  ids.push_back("ga-IE");
  ids.push_back("ml");
  ids.push_back("es-MX");
  ids.push_back("mk");
  ids.push_back("lij");
  ids.push_back("kn");
  ids.push_back("bs");
  ids.push_back("my");
  ids.push_back("ar");
  ids.push_back("gl");
  ids.push_back("hr");
  ids.push_back("hu");
  ids.push_back("nl");
  ids.push_back("bg");
  ids.push_back("es-AR");
  ids.push_back("ast");
  ids.push_back("ka");
  ids.push_back("de");
  ids.push_back("az");
  ids.push_back("gd");
  ids.push_back("br");
  ids.push_back("ko");
  ids.push_back("fi");
  ids.push_back("es-CL");
  ids.push_back("crh");
  ids.push_back("eo");
  ids.push_back("id");
  ids.push_back("fr");
  ids.push_back("et");
  ids.push_back("fa");
  ids.push_back("nn-NO");
  ids.push_back("lt");
  ids.push_back("ff");
  ids.push_back("cy");
  ids.push_back("es-ES");
  ids.push_back("eu");
  ids.push_back("lo");
  ids.push_back("rm");
  ids.push_back("dsb");
  ids.push_back("ta");
  ids.push_back("th");
  ids.push_back("tr");
  ids.push_back("fy-NL");
  ids.push_back("sq");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-us");
  ids.push_back("en_Latn_US");
  ids.push_back("en-us");
  ids.push_back("en_Latn_US");
  ids.push_back("en-us");
  ids.push_back("en_Latn_US");
  ids.push_back("en-us");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-us");
  ids.push_back("en_Latn_US");
  ids.push_back("en-us");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en-US");
  ids.push_back("en");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-us");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en-US");
  ids.push_back("en");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("be");
  ids.push_back("ch");
  ids.push_back("it");
  ids.push_back("ca");
  ids.push_back("uk");
  ids.push_back("au");
  ids.push_back("nl");
  ids.push_back("at");
  ids.push_back("de");
  ids.push_back("fr");
  ids.push_back("es");
  ids.push_back("ie");
  ids.push_back("en-US");
  ids.push_back("be");
  ids.push_back("ch");
  ids.push_back("it");
  ids.push_back("ca");
  ids.push_back("uk");
  ids.push_back("ta");
  ids.push_back("au");
  ids.push_back("ta_Taml_IN");
  ids.push_back("nl");
  ids.push_back("pl");
  ids.push_back("at");
  ids.push_back("pl_Latn_PL");
  ids.push_back("de");
  ids.push_back("es-es");
  ids.push_back("fr");
  ids.push_back("es_Latn_ES");
  ids.push_back("es");
  ids.push_back("en");
  ids.push_back("ie");
  ids.push_back("en-US");
  ids.push_back("hr");
  ids.push_back("en_Latn_US");
  ids.push_back("hr_Latn_HR");
  ids.push_back("en_Latn_US");
  ids.push_back("tr");
  ids.push_back("tr_Latn_TR");
  ids.push_back("hi-in");
  ids.push_back("hi_Deva_IN");
  ids.push_back("ja");
  ids.push_back("ja_Jpan_JP");
  ids.push_back("es-mx");
  ids.push_back("es_Latn_MX");
  ids.push_back("it");
  ids.push_back("it_Latn_IT");
  ids.push_back("zh-tw");
  ids.push_back("zh_Hant_TW");
  ids.push_back("mk");
  ids.push_back("mk_Cyrl_MK");
  ids.push_back("en-us");
  ids.push_back("en_Latn_US");
  ids.push_back("zh-cn");
  ids.push_back("zh_Hans_CN");
  ids.push_back("pt-br");
  ids.push_back("pt_Latn_BR");
  ids.push_back("id");
  ids.push_back("id_Latn_ID");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("fr");
  ids.push_back("fr_Latn_FR");
  ids.push_back("cs");
  ids.push_back("cs_Latn_CZ");
  ids.push_back("hu");
  ids.push_back("hu_Latn_HU");
  ids.push_back("nl");
  ids.push_back("nl_Latn_NL");
  ids.push_back("ro");
  ids.push_back("ro_Latn_RO");
  ids.push_back("el");
  ids.push_back("el_Grek_GR");
  ids.push_back("sr");
  ids.push_back("sr_Cyrl_RS");
  ids.push_back("bn");
  ids.push_back("bn_Beng_BD");
  ids.push_back("de");
  ids.push_back("de_Latn_DE");
  ids.push_back("ru");
  ids.push_back("ru_Cyrl_RU");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("be");
  ids.push_back("ch");
  ids.push_back("it");
  ids.push_back("ca");
  ids.push_back("uk");
  ids.push_back("au");
  ids.push_back("nl");
  ids.push_back("at");
  ids.push_back("de");
  ids.push_back("fr");
  ids.push_back("es");
  ids.push_back("en");
  ids.push_back("ie");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("ja");
  ids.push_back("en-US");
  ids.push_back("ja");
  ids.push_back("en");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("ja");
  ids.push_back("en");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("bn");
  ids.push_back("bn_Beng_BD");
  ids.push_back("cs");
  ids.push_back("cs_Latn_CZ");
  ids.push_back("sl");
  ids.push_back("sk");
  ids.push_back("ur");
  ids.push_back("pa-IN");
  ids.push_back("mai");
  ids.push_back("kab");
  ids.push_back("uz");
  ids.push_back("pl");
  ids.push_back("vi");
  ids.push_back("sq");
  ids.push_back("bn-BD");
  ids.push_back("he");
  ids.push_back("ms");
  ids.push_back("en-US");
  ids.push_back("km");
  ids.push_back("ne-NP");
  ids.push_back("be");
  ids.push_back("da");
  ids.push_back("sv-SE");
  ids.push_back("gn");
  ids.push_back("mr");
  ids.push_back("kk");
  ids.push_back("mn");
  ids.push_back("pt-BR");
  ids.push_back("ja");
  ids.push_back("el");
  ids.push_back("it");
  ids.push_back("ca");
  ids.push_back("zh-TW");
  ids.push_back("nb-NO");
  ids.push_back("cs");
  ids.push_back("ia");
  ids.push_back("te");
  ids.push_back("pt-PT");
  ids.push_back("ach");
  ids.push_back("ru");
  ids.push_back("tl");
  ids.push_back("hi-IN");
  ids.push_back("ro");
  ids.push_back("hsb");
  ids.push_back("zh-CN");
  ids.push_back("cak");
  ids.push_back("hy-AM");
  ids.push_back("gu-IN");
  ids.push_back("su");
  ids.push_back("uk");
  ids.push_back("sr");
  ids.push_back("ga-IE");
  ids.push_back("si");
  ids.push_back("ml");
  ids.push_back("es-MX");
  ids.push_back("mk");
  ids.push_back("lij");
  ids.push_back("kn");
  ids.push_back("bs");
  ids.push_back("my");
  ids.push_back("ar");
  ids.push_back("gl");
  ids.push_back("hr");
  ids.push_back("hu");
  ids.push_back("nl");
  ids.push_back("bg");
  ids.push_back("es-AR");
  ids.push_back("ast");
  ids.push_back("ka");
  ids.push_back("de");
  ids.push_back("az");
  ids.push_back("gd");
  ids.push_back("br");
  ids.push_back("ko");
  ids.push_back("fi");
  ids.push_back("es-CL");
  ids.push_back("crh");
  ids.push_back("eo");
  ids.push_back("id");
  ids.push_back("fr");
  ids.push_back("et");
  ids.push_back("fa");
  ids.push_back("nn-NO");
  ids.push_back("lt");
  ids.push_back("ff");
  ids.push_back("cy");
  ids.push_back("es-ES");
  ids.push_back("eu");
  ids.push_back("lo");
  ids.push_back("rm");
  ids.push_back("dsb");
  ids.push_back("ta");
  ids.push_back("th");
  ids.push_back("tr");
  ids.push_back("fy-NL");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("el");
  ids.push_back("el_Grek_GR");
  ids.push_back("hi-IN");
  ids.push_back("hi_Deva_IN");
  ids.push_back("sl");
  ids.push_back("sk");
  ids.push_back("ur");
  ids.push_back("pa-IN");
  ids.push_back("mai");
  ids.push_back("kab");
  ids.push_back("uz");
  ids.push_back("pl");
  ids.push_back("vi");
  ids.push_back("sq");
  ids.push_back("bn-BD");
  ids.push_back("he");
  ids.push_back("ms");
  ids.push_back("en-US");
  ids.push_back("km");
  ids.push_back("ne-NP");
  ids.push_back("be");
  ids.push_back("da");
  ids.push_back("sv-SE");
  ids.push_back("gn");
  ids.push_back("mr");
  ids.push_back("kk");
  ids.push_back("mn");
  ids.push_back("pt-BR");
  ids.push_back("ja");
  ids.push_back("el");
  ids.push_back("it");
  ids.push_back("ca");
  ids.push_back("zh-TW");
  ids.push_back("nb-NO");
  ids.push_back("cs");
  ids.push_back("ia");
  ids.push_back("te");
  ids.push_back("pt-PT");
  ids.push_back("ach");
  ids.push_back("ru");
  ids.push_back("tl");
  ids.push_back("hi-IN");
  ids.push_back("ro");
  ids.push_back("hsb");
  ids.push_back("zh-CN");
  ids.push_back("cak");
  ids.push_back("hy-AM");
  ids.push_back("gu-IN");
  ids.push_back("su");
  ids.push_back("uk");
  ids.push_back("sr");
  ids.push_back("ga-IE");
  ids.push_back("si");
  ids.push_back("ml");
  ids.push_back("es-MX");
  ids.push_back("mk");
  ids.push_back("lij");
  ids.push_back("kn");
  ids.push_back("bs");
  ids.push_back("my");
  ids.push_back("ar");
  ids.push_back("gl");
  ids.push_back("hr");
  ids.push_back("hu");
  ids.push_back("nl");
  ids.push_back("bg");
  ids.push_back("es-AR");
  ids.push_back("ast");
  ids.push_back("ka");
  ids.push_back("de");
  ids.push_back("az");
  ids.push_back("gd");
  ids.push_back("br");
  ids.push_back("ko");
  ids.push_back("fi");
  ids.push_back("es-CL");
  ids.push_back("crh");
  ids.push_back("eo");
  ids.push_back("id");
  ids.push_back("fr");
  ids.push_back("et");
  ids.push_back("fa");
  ids.push_back("nn-NO");
  ids.push_back("lt");
  ids.push_back("ff");
  ids.push_back("cy");
  ids.push_back("es-ES");
  ids.push_back("eu");
  ids.push_back("lo");
  ids.push_back("rm");
  ids.push_back("dsb");
  ids.push_back("ja");
  ids.push_back("ta");
  ids.push_back("ja_Jpan_JP");
  ids.push_back("th");
  ids.push_back("tr");
  ids.push_back("fy-NL");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("mk");
  ids.push_back("mk_Cyrl_MK");
  ids.push_back("ro");
  ids.push_back("ro_Latn_RO");
  ids.push_back("ru");
  ids.push_back("ru_Cyrl_RU");
  ids.push_back("sr");
  ids.push_back("sr_Cyrl_RS");
  ids.push_back("ta");
  ids.push_back("ta_Taml_IN");
  ids.push_back("zh-CN");
  ids.push_back("zh_Hans_CN");
  ids.push_back("zh-TW");
  ids.push_back("zh_Hant_TW");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("bn");
  ids.push_back("bn_Beng_BD");
  ids.push_back("cs");
  ids.push_back("cs_Latn_CZ");
  ids.push_back("el");
  ids.push_back("el_Grek_GR");
  ids.push_back("hi-IN");
  ids.push_back("hi_Deva_IN");
  ids.push_back("ja");
  ids.push_back("ja_Jpan_JP");
  ids.push_back("mk");
  ids.push_back("mk_Cyrl_MK");
  ids.push_back("ro");
  ids.push_back("ro_Latn_RO");
  ids.push_back("ru");
  ids.push_back("ru_Cyrl_RU");
  ids.push_back("sr");
  ids.push_back("sr_Cyrl_RS");
  ids.push_back("ta");
  ids.push_back("ta_Taml_IN");
  ids.push_back("zh-CN");
  ids.push_back("zh_Hans_CN");
  ids.push_back("zh-TW");
  ids.push_back("zh_Hant_TW");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("en_Latn_US");
  ids.push_back("en");
  ids.push_back("sl");
  ids.push_back("ur");
  ids.push_back("fy-NL");
  ids.push_back("kab");
  ids.push_back("uz");
  ids.push_back("pl");
  ids.push_back("vi");
  ids.push_back("sq");
  ids.push_back("he");
  ids.push_back("ms");
  ids.push_back("km");
  ids.push_back("hy");
  ids.push_back("NN");
  ids.push_back("be");
  ids.push_back("kk");
  ids.push_back("gn");
  ids.push_back("mr");
  ids.push_back("NO");
  ids.push_back("ja");
  ids.push_back("lv");
  ids.push_back("oc");
  ids.push_back("it");
  ids.push_back("ca");
  ids.push_back("is");
  ids.push_back("ia");
  ids.push_back("cz");
  ids.push_back("ru");
  ids.push_back("tl");
  ids.push_back("ro");
  ids.push_back("be-tarask");
  ids.push_back("hsb");
  ids.push_back("zh-CN");
  ids.push_back("pt");
  ids.push_back("uk");
  ids.push_back("sr");
  ids.push_back("ltg");
  ids.push_back("pa");
  ids.push_back("si");
  ids.push_back("ml");
  ids.push_back("kr");
  ids.push_back("mk");
  ids.push_back("an");
  ids.push_back("lij");
  ids.push_back("kn");
  ids.push_back("bs");
  ids.push_back("zh-TW");
  ids.push_back("ka");
  ids.push_back("de");
  ids.push_back("as");
  ids.push_back("az");
  ids.push_back("gd");
  ids.push_back("br");
  ids.push_back("fi");
  ids.push_back("crh");
  ids.push_back("eo");
  ids.push_back("id");
  ids.push_back("fr");
  ids.push_back("sv-SE");
  ids.push_back("es");
  ids.push_back("et");
  ids.push_back("fa");
  ids.push_back("lt");
  ids.push_back("or");
  ids.push_back("cy");
  ids.push_back("eu");
  ids.push_back("lo");
  ids.push_back("tr");
  ids.push_back("wo");
  ids.push_back("sk");
  ids.push_back("da");
  ids.push_back("gu");
  ids.push_back("el");
  ids.push_back("te");
  ids.push_back("ga-IE");
  ids.push_back("my");
  ids.push_back("ar");
  ids.push_back("gl");
  ids.push_back("hr");
  ids.push_back("hu");
  ids.push_back("nl");
  ids.push_back("bg");
  ids.push_back("bn");
  ids.push_back("ne");
  ids.push_back("ast");
  ids.push_back("af");
  ids.push_back("hi");
  ids.push_back("rm");
  ids.push_back("dsb");
  ids.push_back("ta");
  ids.push_back("th");
  ids.push_back("en-US");
  ids.push_back("sl");
  ids.push_back("sk");
  ids.push_back("ur");
  ids.push_back("fy-NL");
  ids.push_back("kab");
  ids.push_back("uz");
  ids.push_back("pl");
  ids.push_back("vi");
  ids.push_back("sq");
  ids.push_back("he");
  ids.push_back("ms");
  ids.push_back("km");
  ids.push_back("hy");
  ids.push_back("NN");
  ids.push_back("be");
  ids.push_back("da");
  ids.push_back("gn");
  ids.push_back("mr");
  ids.push_back("kk");
  ids.push_back("NO");
  ids.push_back("gu");
  ids.push_back("ja");
  ids.push_back("el");
  ids.push_back("lv");
  ids.push_back("oc");
  ids.push_back("it");
  ids.push_back("ca");
  ids.push_back("is");
  ids.push_back("ia");
  ids.push_back("cz");
  ids.push_back("te");
  ids.push_back("ga-IE");
  ids.push_back("ru");
  ids.push_back("tl");
  ids.push_back("ro");
  ids.push_back("be-tarask");
  ids.push_back("hsb");
  ids.push_back("zh-CN");
  ids.push_back("pt");
  ids.push_back("uk");
  ids.push_back("sr");
  ids.push_back("ltg");
  ids.push_back("pa");
  ids.push_back("si");
  ids.push_back("ml");
  ids.push_back("kr");
  ids.push_back("mk");
  ids.push_back("an");
  ids.push_back("lij");
  ids.push_back("kn");
  ids.push_back("bs");
  ids.push_back("zh-TW");
  ids.push_back("my");
  ids.push_back("ar");
  ids.push_back("gl");
  ids.push_back("hr");
  ids.push_back("hu");
  ids.push_back("nl");
  ids.push_back("bg");
  ids.push_back("bn");
  ids.push_back("ne");
  ids.push_back("ast");
  ids.push_back("af");
  ids.push_back("hi");
  ids.push_back("ka");
  ids.push_back("de");
  ids.push_back("as");
  ids.push_back("az");
  ids.push_back("gd");
  ids.push_back("br");
  ids.push_back("fi");
  ids.push_back("crh");
  ids.push_back("eo");
  ids.push_back("id");
  ids.push_back("fr");
  ids.push_back("sv-SE");
  ids.push_back("es");
  ids.push_back("et");
  ids.push_back("en");
  ids.push_back("fa");
  ids.push_back("lt");
  ids.push_back("or");
  ids.push_back("cy");
  ids.push_back("eu");
  ids.push_back("lo");
  ids.push_back("rm");
  ids.push_back("dsb");
  ids.push_back("ta");
  ids.push_back("th");
  ids.push_back("tr");
  ids.push_back("wo");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("sl");
  ids.push_back("sk");
  ids.push_back("ur");
  ids.push_back("fy-NL");
  ids.push_back("kab");
  ids.push_back("uz");
  ids.push_back("pl");
  ids.push_back("vi");
  ids.push_back("sq");
  ids.push_back("he");
  ids.push_back("ms");
  ids.push_back("km");
  ids.push_back("hy");
  ids.push_back("NN");
  ids.push_back("be");
  ids.push_back("da");
  ids.push_back("gn");
  ids.push_back("mr");
  ids.push_back("kk");
  ids.push_back("NO");
  ids.push_back("gu");
  ids.push_back("ja");
  ids.push_back("el");
  ids.push_back("lv");
  ids.push_back("oc");
  ids.push_back("it");
  ids.push_back("ca");
  ids.push_back("is");
  ids.push_back("ia");
  ids.push_back("cz");
  ids.push_back("te");
  ids.push_back("ga-IE");
  ids.push_back("ru");
  ids.push_back("tl");
  ids.push_back("ro");
  ids.push_back("be-tarask");
  ids.push_back("hsb");
  ids.push_back("zh-CN");
  ids.push_back("pt");
  ids.push_back("uk");
  ids.push_back("sr");
  ids.push_back("ltg");
  ids.push_back("pa");
  ids.push_back("si");
  ids.push_back("ml");
  ids.push_back("kr");
  ids.push_back("mk");
  ids.push_back("an");
  ids.push_back("lij");
  ids.push_back("kn");
  ids.push_back("bs");
  ids.push_back("zh-TW");
  ids.push_back("my");
  ids.push_back("ar");
  ids.push_back("gl");
  ids.push_back("hr");
  ids.push_back("hu");
  ids.push_back("nl");
  ids.push_back("bg");
  ids.push_back("bn");
  ids.push_back("ne");
  ids.push_back("ast");
  ids.push_back("af");
  ids.push_back("hi");
  ids.push_back("ka");
  ids.push_back("de");
  ids.push_back("as");
  ids.push_back("az");
  ids.push_back("gd");
  ids.push_back("br");
  ids.push_back("fi");
  ids.push_back("crh");
  ids.push_back("eo");
  ids.push_back("id");
  ids.push_back("fr");
  ids.push_back("sv-SE");
  ids.push_back("es");
  ids.push_back("et");
  ids.push_back("en");
  ids.push_back("fa");
  ids.push_back("lt");
  ids.push_back("or");
  ids.push_back("cy");
  ids.push_back("eu");
  ids.push_back("lo");
  ids.push_back("rm");
  ids.push_back("dsb");
  ids.push_back("ta");
  ids.push_back("th");
  ids.push_back("tr");
  ids.push_back("wo");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-us");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-us");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");
  ids.push_back("en-US");
  ids.push_back("en_Latn_US");

  {
    auto start = chrono::steady_clock::now();

    for (auto& id : ids) {
      icu_64::Locale aLocale = icu_64::Locale(id.c_str());
    }

    
    auto end = chrono::steady_clock::now();
    auto diff = end - start;
    auto measured_us = chrono::duration_cast<chrono::microseconds>(diff).count();
    printf("Create Locale from str for %d locales. time: %d us\n", ids.size(), measured_us);

  }

  {
    // Compare
    icu_64::Locale referenceLocale = icu_64::Locale("en-US");

    for (auto& id : ids) {
      icu_64::Locale aLocale = icu_64::Locale(id.c_str());
      locales.push_back(aLocale);
    }

    unsigned int matches = 0;

    auto start = chrono::steady_clock::now();

    for (auto& loc : locales) {
      if (loc == referenceLocale) {
        matches += 1;
      }
    }
    
    auto end = chrono::steady_clock::now();
    auto diff = end - start;
    auto measured_ns = chrono::duration_cast<chrono::nanoseconds>(diff).count();
    printf("Number of matches against en-US: %d. time: %d ns\n", matches, measured_ns);
  }

  {
    // Size

    // XXX: We suspect that this does not measure heap allocation, leaving `variants` not accounted for.
    // Since there are only 3 locales with variants in the sample, we hope the results
    // remain meaningful, but it would be nice to improve the measuring.

    unsigned int size = sizeof(decltype(locales.back())) * locales.capacity();
    printf("Total size of the locales vector: %d bytes.\n", size);
  }

  {
    // ToString

    UErrorCode status = U_ZERO_ERROR;

    auto start = chrono::steady_clock::now();

    for (auto& loc : locales) {
      string dest = "";
      icu_64::StringByteSink<std::string> sink(&dest);
      loc.toLanguageTag(sink, status);
    }
    
    auto end = chrono::steady_clock::now();
    auto diff = end - start;
    auto measured_us = chrono::duration_cast<chrono::microseconds>(diff).count();
    printf("Serialized Locale. time: %d us\n", measured_us);
  }

  {
    // {Add|Remove}LikelySubtags

    UErrorCode status = U_ZERO_ERROR;

    for (auto& loc : locales) {
      loc.addLikelySubtags(status);
    }

    auto start = chrono::steady_clock::now();

    for (auto& loc : locales) {
      loc.minimizeSubtags(status);
      loc.addLikelySubtags(status);
    }
    
    auto end = chrono::steady_clock::now();
    auto diff = end - start;
    auto measured_us = chrono::duration_cast<chrono::microseconds>(diff).count();
    printf("Added/Removed likely subtags. time: %d us\n", measured_us);
  }
}


int main() {
  show();
  return 0;
}
