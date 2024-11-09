let canvas = document.getElementById('myCanvas');
let ctx;
if (canvas.getContext) {
    ctx = canvas.getContext('2d');

    // draw here
    ctx.fillStyle = 'green';
    ctx.fillRect(0, 0, 500, 500);
} else {
    console.log("Canvas not supported");
}

fetch('/api/grid/100', {
    method: 'GET' // or 'POST'
}).then((response) => {
    if (response.ok) {
        return response.json(); // Here we're using JSON but you can use other formats such as blob, text etc.
    } else {
        throw new Error('Server response wasn\'t OK');
    }
}).then((grid) => {
    const cell_factor = 500 / grid.size;
    ctx.font = `5px Arial`;
    ctx.fillStyle = "grey"
    for (let r = 0; r < grid.size; r++) {
        for (let c = 0; c < grid.size; c++) {
            ctx.fillText("" + grid.grid[r][c], 5 + c * cell_factor, r * cell_factor);
        }
    }
}).catch((err) => {
    console.log('Fetching failed', err);
});