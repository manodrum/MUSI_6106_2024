window.handle = null

import("./pkg").catch(console.error).then(rust_module => {
    const play_button = document.getElementById("play")
    play_button.addEventListener("click", event => {
        window.handle = rust_module.play()
    })
    const stop_button = document.getElementById("stop")
    stop_button.addEventListener("click", event => {
        if (window.handle !== null) {
            window.handle.free()
	        window.handle = null
        }
    })
    const delay_slider = document.getElementById("delay")
    delay_slider.addEventListener("change", event => {
        if (window.handle !== null) {
            const delay = +delay_slider.value
            console.log("delay", delay)
            window.handle.set_delay(delay)
        }
    })
    const gain_slider = document.getElementById("delay")
    gain_slider.addEventListener("change", event => {
        if (window.handle !== null) {
            const gain = +gain_slider.value
            console.log("gain", gain)
            window.handle.set_gain(gain)
        }
    })
})
