const shorten_textfield = document.getElementById("shorten_textfield");
const shorten_button = document.getElementById("shorten_button");

let copy_clipboard = false;

let hide_advanced = true;

// Add keylistener to the textfield so we can also submit on enter
shorten_textfield.addEventListener("keypress", function (event) {
	if (event.key === "Enter") {
		event.preventDefault();
	}
});

shorten_button.onclick = handle_shorten_click;

document.getElementById("advanced_mode").addEventListener("click", function () {
	let inputs = document.getElementsByClassName('advanced_mode');
	hide_advanced = !hide_advanced;
	for (let i = 0; i < inputs.length; i++) {
		if (hide_advanced) {
			inputs[i].style.display = "none";
		} else {
			inputs[i].style.display = "inherit";
		}
	}
});

function handle_shorten_click() {
	if (copy_clipboard === true) {
		navigator.clipboard.writeText(shorten_textfield.value);
		shorten_textfield.value = "";
		shorten_button.value = "Shorten"
	} else {
		if (shorten_textfield.value !== "") {
			const xhr = new XMLHttpRequest();
			const url = "http://localhost:7999/custom";
			xhr.open("POST", url, true);
			xhr.setRequestHeader("Content-Type", "application/json");
			xhr.onreadystatechange = function () {
				// Check if request is completed
				if (xhr.readyState !== 4) {
					return;
				}

				// Response successful
				if (xhr.status === 200) {
					handle_success(xhr.responseText)
				} else if (xhr.status === 409) { // Conflict
					handle_conflict();
				}
			}

			const to_shorten = shorten_textfield.value;
			const data = JSON.stringify({"link": to_shorten});
			xhr.send(data);
		}
	}

	copy_clipboard = !copy_clipboard;
}


function handle_success(response) {
	// console.log(response);
	let shorten_btn = document.getElementById("shorten_button");
	shorten_btn.value = "Copy to clipboard";
	copy_clipboard = true;
	shorten_textfield.value = response;
}

function handle_conflict() {

}