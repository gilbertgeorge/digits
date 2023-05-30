import {Digits} from "digits";

const solveButton = document.getElementById("solve");
const clearButton = document.getElementById("clear");
const number1Input = document.getElementById("num1");
const number2Input = document.getElementById("num2");
const number3Input = document.getElementById("num3");
const number4Input = document.getElementById("num4");
const number5Input = document.getElementById("num5");
const number6Input = document.getElementById("num6");
const targetValue = document.getElementById("targetValue");
const answerList = document.getElementById("answers");

solveButton.addEventListener("click", event => {
    let startDigits = new Uint32Array(6);
    startDigits[0] = number1Input.value;
    startDigits[1] = number2Input.value;
    startDigits[2] = number3Input.value;
    startDigits[3] = number4Input.value;
    startDigits[4] = number5Input.value;
    startDigits[5] = number6Input.value;

    startDigits = startDigits.filter(function (el) {
        return el != 0;
    });
    let digits = Digits.new(targetValue.value, startDigits);
    let answers = digits.solve();
    console.log(`target: ${targetValue.value}`)
    console.log(`startDigits: ${startDigits}`);
    console.log(answers);

    while (answerList.firstChild) {
        answerList.removeChild(answerList.firstChild);
    }
    for (let i = 0; i < answers.length; i++) {
        let answerDiv = document.createElement("answer");
        answerDiv.innerHTML = answers[i].join(" ↵ ");
        answerList.appendChild(answerDiv);
    }

});

clearButton.addEventListener("click", event => {
    number1Input.value = null;
    number2Input.value = null;
    number3Input.value = null;
    number4Input.value = null;
    number5Input.value = null;
    number6Input.value = null;
    targetValue.value = null;
    while (answerList.firstChild) {
        answerList.removeChild(answerList.firstChild);
    }
});

number1Input.addEventListener("input", event => {
    if (event.target.value) {
        number1Input.value = event.target.value;
    } else {
        number1Input.value = 0;
    }
});