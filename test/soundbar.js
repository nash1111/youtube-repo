let mic,
  micList,
  micStarted,
  fft = false;
let bars = []; // サウンドバーを格納する配列
let numBars = 250; // サウンドバーの数

function setup() {
  createCanvas(1800, 920);
  mic = new p5.AudioIn();

  mic.getSources(function (sources) {
    micList = createSelect();
    micList.position(10, 10);

    for (let i = 0; i < sources.length; i++) {
      micList.option(sources[i].label, i);
    }

    micList.changed(function () {
      let micIndex = micList.value();
      mic.setSource(micIndex);
      if (!micStarted) {
        userStartAudio().then(() => {
          mic.start();
          micStarted = true;

          fft = new p5.FFT();
          fft.setInput(mic);
        });
      }
    });
  });

}

function draw() {
    background(0);
  
    if (micStarted) {
      let vol = mic.getLevel();
      let spectrum = fft.analyze();
      for (let i = 0; i < numBars; i++) {
        let amplitude = spectrum[i];
        let y = map(amplitude, 0, 256, 0, height); // 高さをキャンバスの高さにマッピング
        let col = color(map(i, 0, numBars, 0, 255), 100, 255 - map(i, 0, numBars, 0, 255));
        fill(col);
        let w = width / numBars;
        rect(i * w, height - y, w - 2, y); // Y座標をキャンバスの高さからバーの高さを引いた値に設定
      }
    }
  }
  
