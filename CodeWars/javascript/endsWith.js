function solution(str, ending) {
    if (ending === "") {
        return true;
    }
    const lastPart = str.slice(ending.length * -1);
    return ending === lastPart;
}

function main() {
    solution("abc", "bc") ? console.log("success") : console.log("fail");
}

main();
