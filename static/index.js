const form = document.getElementById("shorten-url");
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

            alert(`your short url is: ${document.documentURI}${jsonResponse.short_url}`)
        } catch (e) {
            console.error(e);
            alert("Something went wrong :')")
        }
    })
}