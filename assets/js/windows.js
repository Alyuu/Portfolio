document.addEventListener("DOMContentLoaded", main, false);

function main() {
	document.querySelectorAll('.close').forEach(button => {
		button.addEventListener('click', toggleWindow);
	});
}

function toggleWindow(e) {
	e.currentTarget
		.parentNode
		.classList
		.toggle('closed');
}
