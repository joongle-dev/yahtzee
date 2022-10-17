import { initializeApp } from 'https://www.gstatic.com/firebasejs/9.12.1/firebase-app.js'
import { getDatabase, ref, set, get, push, child, onValue } from 'https://www.gstatic.com/firebasejs/9.12.1/firebase-database.js'
import init, { Yahtzee, Network, send_sdp_offer, answer_sdp_offer, receive_sdp_answer } from './yahtzee.js'

const codeChars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

function generateCode(length) {
    var code = "";
    for (var i = 0; i < length; i++) {
        code += codeChars.charAt(Math.floor(Math.random() * 36));
    }
    return code;
}

const firebaseConfig = { 
    apiKey: "AIzaSyAWCsmtbnCbitS7SS43gRuGYTsQRtCnIA8",
    authDomain: "joongle-447b0.firebaseapp.com",
    databaseURL: "https://joongle-447b0-default-rtdb.firebaseio.com",
    projectId: "joongle-447b0",
    storageBucket: "joongle-447b0.appspot.com",
    messagingSenderId: "44823880286",
    appId: "1:44823880286:web:3d01a2cc5f337edf8c4acf",
    measurementId: "G-75Q58WCSEL"
};

await init();

const app = initializeApp(firebaseConfig);
const database = getDatabase(app);

const menuForm = document.getElementById("menu-form");
const hostForm = document.getElementById("host-form");
const joinForm = document.getElementById("join-form");

const hostMenuBtn = document.querySelector(".host-menu-btn");
const joinMenuBtn = document.querySelector(".join-menu-btn");
const hostBtn = document.querySelector(".host-btn");
const joinBtn = document.querySelector(".join-btn");
const inviteCode = document.querySelector(".invite-code");

const joinCode = document.querySelector(".join-code");
const userId = generateCode(6);
let network = new Network(userId);

console.log(userId);

hostMenuBtn.addEventListener("click", (e) => {
    e.preventDefault();
    hostForm.style.display = "none";
    menuForm.style.display = "block";
});

joinMenuBtn.addEventListener("click", (e) => {
    e.preventDefault();
    joinForm.style.display = "none";
    menuForm.style.display = "block";
});

function joinSession(session) {
    set(session, `{"type":"join", "id":"${userId}"}`);
    onValue(session, async (snapshot) => {
        const value = JSON.parse(snapshot.val());
        switch (value.type) {
            case "join":
                if (value.id !== userId) {
                    console.log(`sdp offer to ${value.id}`);
                    var sdp = (await send_sdp_offer(network, value.id)).replace(/(\r\n|\n|\r)/gm, "\\n");
                    set(session, `{"type":"sdp-offer", "id":"${userId}", "target":"${value.id}", "sdp":"${sdp}"}`);
                }
                break;
            case "sdp-offer":
                if (value.target === userId) {
                    console.log(`sdp offer from ${value.id}`);
                    const sdp = (await answer_sdp_offer(network, value.id, value.sdp)).replace(/(\r\n|\n|\r)/gm, "\\n");
                    set(session, `{"type":"sdp-answer", "id":"${userId}", "target":"${value.id}", "sdp":"${sdp}"}`);
                }
                break;
            case "sdp-answer":
                if (value.target === userId) {
                    console.log(`sdp answer from ${value.id}`);
                    await receive_sdp_answer(network, value.id, value.sdp);
                }
                break;
            default:
                console.log(value.type);
        }
    });
}

hostBtn.addEventListener("click", (e) => {
    e.preventDefault();
    menuForm.style.display = "none";
    hostForm.style.display = "block";
    const sessionId = generateCode(5);
    inviteCode.textContent = sessionId;
    const sessionRef = ref(database, `sessions/${sessionId}`);
    joinSession(sessionRef);
});

joinBtn.addEventListener("click", (e) => {
    e.preventDefault();
    menuForm.style.display = "none";
    joinForm.style.display = "block";
});

joinForm.addEventListener("submit", (e) => {
    e.preventDefault();
    if (joinCode.value.length === 5) {
        console.log(joinCode.value);
        const sessionRef = ref(database, `sessions/${joinCode.value}`);
        joinSession(sessionRef);
        /*
        get(sessionRef).then((snapshot) => {
                joinSession(sessionRef);
            }).catch((error) => {
                console.error(error);
            }
        );
        */
    }
});

joinCode.addEventListener("keypress", (e) => {
    if (e.keyCode !== 13) {
        e.preventDefault();
    }
    var character = String.fromCharCode(e.keyCode).toUpperCase();
    if (joinCode.value.length < 5 && codeChars.indexOf(character) != -1) {
        joinCode.value += character;
    }
});