function longest(s1, s2) {
    const uniques = new Set([...new Set(s1), ...new Set(s2)]);
    return [...uniques].sort().join("");
}

function main() {
    const answer = longest("xyaabbbccccdefww", "xxxxyyyyabklmopq");
    console.log(answer);
}

main();
