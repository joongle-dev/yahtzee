import { initializeApp } from 'https://www.gstatic.com/firebasejs/9.10.0/firebase-app.js'
import { getDatabase } from 'https://www.gstatic.com/firebasejs/9.10.0/firebase-database.js'

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

const app = initializeApp(firebaseConfig);
const database = getDatabase(app);

const hostMenuBtn = document.querySelector(".host-menu-btn");
const joinMenuBtn = document.querySelector(".join-menu-btn");
const hostBtn = document.querySelector(".host-btn");
const joinBtn = document.querySelector(".join-btn");
const menuForm = document.getElementById("menu-form");
const hostForm = document.getElementById("host-form");
const joinForm = document.getElementById("join-form");

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

hostBtn.addEventListener("click", (e) => {
    e.preventDefault();
    menuForm.style.display = "none";
    hostForm.style.display = "block";
});

joinBtn.addEventListener("click", (e) => {
    e.preventDefault();
    menuForm.style.display = "none";
    joinForm.style.display = "block";
});