const {app, BrowserWindow} = require('electron')

let window

function createWindow() {
  window = new BrowserWindow({width: 480, height: 360})

  window.loadFile('./arkanoidFiles/BasicArkanoidClone.html')

  // window.webContents.openDevTools()

  window.on('closed', () => {
    window = null
  })
}



app.on('ready', createWindow)