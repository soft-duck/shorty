const shorten_textfield = document.getElementById("shorten_textfield");
const shorten_button = document.getElementById("shorten_button");

// Add keylistener to the textfield so we can also submit on enter
shorten_textfield.addEventListener("keypress", function (event) {
	if (event.key === "Enter") {
		event.preventDefault();
		shorten_button.click();
	}
})

shorten_button.onclick = handle_shorten_click;

function handle_shorten_click() {
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


function handle_success(response) {
	console.log(response);
	shorten_textfield.value = response;
}

function handle_conflict() {

}