import {Digits} from "digits";

const solveButton = document.getElementById("solve");
const number1Input = document.getElementById("num1");
const number2Input = document.getElementById("num2");
const number3Input = document.getElementById("num3");
const number4Input = document.getElementById("num4");
const number5Input = document.getElementById("num5");
const number6Input = document.getElementById("num6");
const targetValue = document.getElementById("targetValue");
const answerList = document.getElementById("answers");
// let startDigits = Uint32Array.from([1,2,3,4,5,10]);
// let digits = Digits.new(51, startDigits);
// console.log(digits.solve());

solveButton.addEventListener("click", event => {
    // let startDigits = Uint32Array.from([1,2,3,4,5,10]);
    let startDigits = new Uint32Array(6);
    startDigits[0] = number1Input.value;
    startDigits[1] = number2Input.value;
    startDigits[2] = number3Input.value;
    startDigits[3] = number4Input.value;
    startDigits[4] = number5Input.value;
    startDigits[5] = number6Input.value;
    // let digits = Digits.new(51, startDigits);

    //remove 0 from startDigits
    startDigits = startDigits.filter(function (el) {
        return el != 0;
    });
    let digits = Digits.new(targetValue.value, startDigits);
    let answers = digits.solve();
    console.log(`target: ${targetValue.value}`)
    console.log(`startDigits: ${startDigits}`);
    console.log(answers);

    // add div to answerList for each answer
    //answerList.innerHTML = "";
    for (let i = 0; i < answers.length; i++) {
        let answerDiv = document.createElement("answer");
        answerDiv.innerHTML = answers[i].join(" ↵ ");
        answerList.appendChild(answerDiv);
    }

});

number1Input.addEventListener("input", event => {
    if (event.target.value) {
        number1Input.value = event.target.value;
    } else {
        number1Input.value = 0;
    }
});