var canvas = document.getElementById("gameBoard");
var ctx = canvas.getContext("2d");
var x = canvas.width / 2;
var y = canvas.height - 30;
var dx = 2;
var dy = -2;
var ballRadius = 10;
var platformHeight = 10;
var platformWidth = 100;
var platformX = (canvas.width - platformWidth) / 2;
var rightIsPressed = false;
var leftIsPressed = false;
var brickRows = 3;
var brickColumns = 5;
var brickwWidth = 75;
var brickHeight = 20;
var brickPadding = 10;
var brickOffsetTop = 30;
var brickOffsetLeft = 30;
var score = 0;
var extraLives = 3;

var bricks = [];

for(i = 0; i < brickColumns; i++) {
    bricks[i] = [];
    for(j = 0; j < brickRows; j++) {
        bricks[i][j] = { x: 0, y: 0, status: 1 };
    }
}

document.addEventListener("keydown", keyDownHandler, false);
document.addEventListener("keyup", keyUpHandler, false);

function keyDownHandler(e) {
    if(e.keyCode == 39) {
        rightIsPressed = true;
    } else if(e.keyCode == 37) {
        leftIsPressed = true;
    }
}

function keyUpHandler(e) {
    if(e.keyCode == 39) {
        rightIsPressed = false;
    } else if(e.keyCode == 37) {
        leftIsPressed = false;
    }
}

function moveThePlatform() {
    if(rightIsPressed && platformX < canvas.width - platformWidth) {
        platformX += 7;
    } else if(leftIsPressed && platformX > 0) {
        platformX -= 7;
    }

    x += dx;
    y += dy;
}

function collisionDetection() {
    for(i = 0; i < brickColumns; i++) {
        for(j = 0; j < brickRows; j++) {
            var b = bricks[i][j];
            if(b.status == 1) {
                if(x > b.x && x < b.x + brickwWidth && y > b.y && y < b.y + brickHeight) {
                    dy = -dy;
                    platformWidth -= 5; // platformen blir 5px mindre for hvert mål du treffer.
                    //dy++; speeds up the ball when it hit a brick!
                    b.status = 0;
                    score += 10;
                    if((score / 10) == brickRows * brickColumns) {
                        alert("DU VANT, GRATULERER SÅ MYE!\n" + "DIN POENGSUM BLE: " + score);
                        document.location.reload();
                    }
                }
            }
        }
    }
}

function drawBall() {
    ctx.beginPath();
    ctx.arc(x, y, ballRadius, 0, Math.PI*2);
    ctx.fillStyle = "#0095DD";
    ctx.fill();
    ctx.closePath();
}

function drawplatform() {
    ctx.beginPath();
    ctx.rect(platformX, canvas.height - platformHeight, platformWidth, platformHeight);
    ctx.fillStyle = "#0095DD";
    ctx.fill();
    ctx.closePath();
}

function drawBricks() {
    for(i = 0; i < brickColumns; i++) {
        for(j = 0; j < brickRows; j++) {
            if(bricks[i][j].status == 1){
                var brickX = (i * (brickwWidth + brickPadding)) + brickOffsetLeft;
                var brickY = (j * (brickHeight + brickPadding)) + brickOffsetTop;
                bricks[i][j].x= brickX;
                bricks[i][j].y = brickY;
                ctx.beginPath();
                ctx.rect(brickX, brickY, brickwWidth, brickHeight);
                ctx.fillStyle = "#FF0000";
                ctx.fill();
                ctx.closePath();
            }
        }
    }
}

function drawScore() {
    ctx.font = "16px Arial";
    ctx.fillStyle = "#000";
    ctx.fillText("Score: " + score, 8, 20);
}

function drawExtraLives() {
    ctx.font = "16px Arial";
    ctx.fillStyle = "#000";
    ctx.fillText("Ekstra liv: " + extraLives, canvas.width - 100, 20);
}

function gameController() {
    drawBall();
    drawplatform();
    drawBricks();
    drawScore();
    drawExtraLives();
    collisionDetection();
    
    moveThePlatform();

    if(x + dx > canvas.width - ballRadius || x + dx < ballRadius) {
        dx = -dx;
    } if(y + dy < ballRadius) {
        dy = -dy;
    } else if(y + dy > canvas.height - ballRadius) {
        if(x > platformX && x < platformX + platformWidth) {
            dy = -dy;
        } else {
            extraLives--;
            if(!extraLives) {
                alert("SPILLET ER SLUTT, DU ER TOM FOR EKSTRALIV!\n" + "DIN POENGSUM BLE: " + score);
                document.location.reload();
            } else {
                x = canvas.width / 2;
                y = canvas.height - 30;
                dx = 3;
                dy = -3;
                platformX = (canvas.width-platformWidth) / 2;
            }
        }
    } 
}

function draw() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    gameController();

    requestAnimationFrame(draw);
}

draw();