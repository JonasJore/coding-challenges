function bump(x) {
    let bumpArr = x.split(''); let bumpsHit = 0; let i = 0; while(i < bumpArr.length) { if(bumpArr[i] === 'n') { ++bumpsHit; } ++i; } return(bumpsHit > 15) ? 'Car Dead' : 'Woohoo!';
}

bump("_____n_n_____nnnn_n")