const shorten_class = "shorten";
const copy_class = "copy";
const copied_class = "copied";
const validation_class = "validatable";

const error = "error";
const warning = "warning";
const info = "info";

const shortenField = document.getElementById("shorten_textfield");
const shortenButton = document.getElementById("shorten_button");
const advancedMode = document.getElementById("advanced_mode");
const maxUses = document.getElementById("max_uses");
const ageDays = document.getElementById("age_days");
const advancedInputs = document.getElementsByClassName('advanced_mode');
const customIdField = document.getElementById("custom_id");
const boxList = document.getElementById("box_list");
const messageBox = document.getElementById("message_box");
const duration = document.getElementById("duration");
const dateToggle = document.getElementById("date_toggle");
const shortenButtonText = shortenButton.value;

duration.value = "00:00:00:00";
// ageDays.style.display = "inherit";
// duration.style.display = "none";

// set constraints on field
{
	let xhr = new XMLHttpRequest();
	let configLocation = getEndpointUrl("/config");
	xhr.open('GET', configLocation, true);
	xhr.responseType = 'json';
	xhr.onload = () => {
		if (xhr.status === 200) {
			handleConfig(xhr.response)
		} else {
			message(xhr.status + ": " + xhr.responseText);
		}
	};
	xhr.send();
}

maxUses.max = 2 ** 63 - 1;
maxUses.min = 0;

// sets the date input minimum date to exactly now
{
	const date = new Date();
	date.setMilliseconds(date.getMilliseconds() - (date.getTimezoneOffset() * 1000 * 60));
	ageDays.min = date.toISOString().split(".")[0];
}

// Add keylistener to the textfield so we can also submit on enter
shortenField.addEventListener("keypress", preventDefaultEnter);
shortenField.oninput = () => {
	shortenField.classList.add(validation_class);
	setButtonMode(shorten_class);
};
shortenField.onfocus = () => {
	shortenField.classList.add(validation_class);
}
//shortenField.addEventListener("paste", () => { setButtonMode(shorten_class); });
shortenButton.addEventListener("click", handleShortenClick);
advancedMode.addEventListener("click", advancedModeSwitchHandler);

dateToggle.addEventListener("click", (event) => {
	if (dateToggle.checked) {
		ageDays.classList.remove("invisible");
		duration.classList.add("invisible");
	} else {
		ageDays.classList.add("invisible");
		duration.classList.remove("invisible");
	}

	ageDays.classList.toggle("disappear");
	duration.classList.toggle("disappear");
})

duration.addEventListener("keydown", durationControl)
duration.addEventListener("cut", durationCut)
duration.addEventListener("paste", durationPaste)

function handleShortenClick(event) {
	shortenField.classList.add(validation_class);

	if (getButtonMode() === copy_class) {
		navigator.clipboard.writeText(shortenField.value);
		setButtonMode(copied_class);
		setTimeout(() => {
			if (getButtonMode() === copy_class) {
				setButtonMode(copy_class);
			}
		}, 2000);
		return;
	}

	if (!((advancedFieldsValid() || !advancedMode.checked) && shortenField.checkValidity())) {
		let invalidFields = "";
		let numberWords = ["is", "it"];

		if (!shortenField.checkValidity()) {
			invalidFields += "url ";
		}
		// invalidFields = invalidFields.charAt(0).toUpperCase() + invalidFields.slice(1);
		// invalidFields = invalidFields.slice(0, -2);

		if (!maxUses.checkValidity() && advancedMode.checked) {
			invalidFields += "and max usages";
			numberWords = ["are", "them"]
		}

		message(invalidFields + " " + numberWords[0] + " invalid, check " + numberWords[1] + " and try again.", error);

		return;
	}

	clearMessages();

	const xhr = new XMLHttpRequest();
	const url = getEndpointUrl("/custom");
	xhr.open("POST", url, true);
	xhr.setRequestHeader("Content-Type", "application/json");
	xhr.onreadystatechange = () => {
		// Check if request is completed
		if (xhr.readyState !== 4) {
			return;
		}
		// Response successful
		if (xhr.status === 200) {
			handleSuccess(xhr.responseText)
		} else if (xhr.status === 409) { // Conflict
			handleConflict();
		} else {
			message(xhr.status + ": " + xhr.responseText);
		}
	}

	const data = {};
	data["link"] = shortenField.value;

	if (maxUses.value !== '') {
		data["max_uses"] = -0;
	}

	if (ageDays.value !== '') {
		let validFor;
		if (dateToggle.checked) {
			let now = new Date();
			let date = Date.parse(ageDays.value);
			validFor = date - now.getTime();
		} else {
			validFor = getDurationSeconds() * 1000;
		}
		data["valid_for"] = validFor;
	}

	if (customIdField.value !== '') {
		data["custom_id"] = customIdField.value;
	}

	xhr.send(JSON.stringify(data))
}

function advancedFieldsValid() {
	return ageDays.checkValidity() && maxUses.checkValidity() && customIdField.checkValidity();
}

function getButtonMode() {
	const classes = shortenButton.className.split(" ");

	let mode = classes.find((c) => {
		return c === shorten_class || c === copy_class || c === copied_class;
	});

	if (mode === copied_class) {
		mode = copy_class;
	}

	return mode;
}

function setButtonMode(mode) {
	shortenButton.classList.remove(shorten_class, copy_class, copied_class);
	shortenButton.classList.add(mode);

	let text;

	if (mode === copy_class) {
		text = "Copy";
	} else if (mode === shorten_class) {
		text = shortenButtonText;
	} else if (mode === copied_class) {
		text = "Copied!";
	}

	shortenButton.value = text;
}

function preventDefaultEnter(event) {
	if (event.key === "Enter") {
		event.preventDefault();
		return;
	}
}

function getEndpointUrl(endpoint) {
	return window.location.origin + endpoint;
}

function advancedModeSwitchHandler(event) {
	for (let i = 0; i < advancedInputs.length; i++) {
		if (advancedMode.checked) {
			if (
				advancedInputs[i] === ageDays
				&& !dateToggle.checked
				|| advancedInputs[i] === duration
				&& dateToggle.checked
			) {
				continue
			}
			advancedInputs[i].classList.remove("invisible");
		} else {
			advancedInputs[i].classList.add("invisible");
		}
	}
}

function message(message, type) {
	const box = messageBox.content.firstElementChild.cloneNode(true);
	box.classList.add(type)
	box.innerText = message;
	boxList.appendChild(box)
}

function clearMessages() {
	boxList.replaceChildren();
}

function handleConfig(config) {
	shortenField.maxLength = config.max_link_length;
	customIdField.maxLength = config.max_custom_id_length;
}

function handleSuccess(response) {
	setButtonMode(copy_class);
	shortenField.value = response;
}

function handleConflict() {
	message("Custom Id:" + customIdField.value + " already used. Try something different", error);
}

function getDurationSeconds() {
	let durations = duration.value.split(":");
	return durations[0] * 86400 + durations[1] * 3600 + durations[2] * 60 + durations[3];
}

// Duration field

function durationControl(event) {
	// permit arrow keys
	if ((event.keyCode > 40 || event.keyCode < 37) && !(event.ctrlKey && [86, 67, 88].includes(event.keyCode))) {
		event.preventDefault();
	}

	let cursor = duration.selectionStart;

	if (cursor === 0 && event.keyCode !== 46) {
		return;
	}

	let newValue = duration.value;
	let key = "0";
	let offset = -1;

	// numbers 0 - 9
	if (event.keyCode > 47 && event.keyCode < 59) {
		key = event.key;
	// delete key
	} else if (event.keyCode === 46 && cursor !== 11) {
		offset = 1;
	// backspace
	} else if (event.keyCode !== 8) {
		return;
	}

	if (newValue[cursor + Math.min(offset, 0)] === ":") {
		cursor += offset;
	}

	newValue = newValue.substring(0, cursor + Math.min(offset, 0)) + key + newValue.substring(cursor + Math.max(offset, 0), newValue.length);
	cursor += offset;

	duration.value = newValue;
	duration.selectionStart = duration.selectionEnd = cursor;
}

function durationCut(event) {
	event.preventDefault();

	let end = duration.selectionEnd;
	let text = duration.value;
	let selected = text.slice(duration.selectionStart, end);
	navigator.clipboard.writeText(selected);

	text = text.slice(0, duration.selectionStart) + selected.replaceAll(/\d/g, '0') + text.slice(end)

	duration.value = text;
	duration.selectionStart = duration.selectionEnd = end;
}

function durationPaste(event) {
	event.preventDefault();

	let paste = (event.clipboardData || window.clipboardData).getData('text');

	if (!/^[\d:]+$/.test(paste)) {
		return
	}

	paste = paste.replaceAll(":", "");

	let cursor = duration.selectionEnd

	paste = paste.split("").reverse().join("").slice(0, cursor - Math.floor(cursor / 3));

	let text = duration.value;

	for (let char of paste) {
		if (text[cursor - 1] === ':') {
			cursor--;
		}

		text = text.slice(0, cursor - 1) + char + text.slice(cursor);

		cursor--;
	}

	duration.value = text;
}
