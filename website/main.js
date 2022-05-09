const shorten_textfield = document.getElementById("link_shorten_textfield");
const shorten_button = document.getElementById("link_shorten_button");

// Add keylistener to the textfield so we can also submit on enter
shorten_textfield.addEventListener("keypress", function (event) {
    if (event.key === "Enter") {
        event.preventDefault();
        shorten_button.click();
    }
})

function handle_submit() {
    console.log("sus");
}