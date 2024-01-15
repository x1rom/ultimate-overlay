const url = "http://localhost:4875/get_data";
const polling_interval = 500;

//run once, then poll every x milliseconds
fetchAndUpdate();
window.setInterval(async () => {
    fetchAndUpdate();
}, polling_interval);

async function fetchAndUpdate() {
    update(await getData());
}

//fetch data from controller
async function getData() {
    const response = await fetch(url, {
        method: "GET",
        mode: "cors",
    });
    const data = JSON.parse(await response.text());
    return data;
}

function update(data) {
    //scoreboard
    const score_l = document.getElementById("score_l");
    const score_r = document.getElementById("score_r");
    const name_l = document.getElementById("name_l");
    const name_r = document.getElementById("name_r");
    const tournament_name = document.getElementById("tournament_name");
    const round_name = document.getElementById("round_name");
    if (score_l) score_l.innerText = data.score_l;
    if (score_r) score_r.innerText = data.score_r;
    if (name_l) name_l.innerText = data.name_l;
    if (name_r) name_r.innerText = data.name_r;
    if (tournament_name) tournament_name.innerText = data.tournament_name;
    if (round_name) round_name.innerText = data.round_name;

    //caster
    const caster1 = document.getElementById("caster1");
    const caster2 = document.getElementById("caster2");
    if (caster1) caster1.innerText = data.casters[0];
    if (caster2) caster2.innerText = data.casters[1];
}
