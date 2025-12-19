

document.getElementById('Signup_Form').addEventListener('submit', async (event) => {
    event.preventDefault();

    const form_data = new FormData(event.target);
    const data = Object.fromEntries(form_data.entries());

    try {
        const response = await fetch('/signup', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(data),
        });
        if (response.ok) {
            const result = await response.json();
            console.log(response);
            alert("Login successfull");
            // window.location.href = "/vault";
        } else {
            alert("Error: ", response.statusText);
        }
    } catch (error) {
        console.log("Error submiting form:", error);
    }
});