#include <gmp.h>
#include <iostream>
#include "dec.h"

using namespace std;

const char* N_str =
    "10715086071862673209484250490600018105614048117055336074437503883703510511"
    "24936122493198378815695858127594672917553146900293377082438286592673040090"
    "27987431371873358107053098846355341597977322595205943373851868976298683624"
    "14475309001507719259272508669419676508606630823351242964205044695669333236"
    "417591";

const char* e_str =
    "10335071977839588495324343307012721241868030345867699233451500809021555989"
    "40302810374322178241744090084840310224701201287590526851878584567875669692"
    "57140079887782687520260492762810253290380710870214468348565666875377299183"
    "72863729292015978809506607411711073716898691660211835403800810547133032654"
    "209857";

const char* c_star_s =
    "77578956825544771401324791883447519867965391774167533692559933526520559797"
    "45568787966196883914901534005536907151568251864100834672394418679303623687"
    "59072824742512821423959166270736914130604102452801162684877374802075310241"
    "07902698664117607932987143144840434115330795749666874995701111872117286699"
    "6397";

// const char *m_text_s = "2";

    const char* c_cipher = "775789568255447714013247918834475198679653917741675336925599335265205597974556878796619688391490153400553690715156825186410083467239441867930362368759072824742512821423959166270736914130604102452801162684877374802075310241079026986641176079329871431448404341153307957496668749957011118721172866996397";

int main() {
    char* m = dec(c_cipher);  // access the dec oracle

    printf("%s", m);

    return 0;
}
