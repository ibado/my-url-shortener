const form = document.getElementById("shorten-url");
const dialog = document.getElementById("short-link-dialog");
const content = dialog.getElementsByClassName("content")[0];
const btnCopy = dialog.getElementsByClassName("btn-copy")[0];

btnCopy.addEventListener("click", () => {
    navigator.clipboard.writeText(content.innerHTML).then(() => alert("Copied to clipboard!"))
})
if (form) {
    form.addEventListener("submit", async (event) => {
        event.preventDefault();
        console.log("submitting form...");
        const urlValue = new FormData(form).get("url");
        try {
            const rawResponse = await fetch(
                "/url",
                {
                    method: "POST",
                    headers: {"Accept": "application/json", "Content-Type": "application/json"},
                    body: JSON.stringify({url: urlValue}),
                    mode: "cors"
                }
            );

            const jsonResponse = await rawResponse.json()
            console.log(jsonResponse);

            content.innerHTML = `${document.documentURI}${jsonResponse.short_url}`
            dialog.showModal()
        } catch (e) {
            console.error(e);
        }
    })
}