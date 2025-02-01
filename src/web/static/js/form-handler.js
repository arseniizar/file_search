document.addEventListener('DOMContentLoaded', function () {
    console.log("DOM fully loaded");
    const form = document.getElementById('add-file-form');
    if (form) {
        form.addEventListener('submit', function (event) {
            event.preventDefault();
            const data = {
                path: form.path.value,
                name: form.name.valueOf,
                modified_time: form.modified_time.value
            };
            fetch(form.action, {
                method: form.method,
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify(data)
            })
                .then(response => response.json())
                .then(json => console.log(json))
                .catch(err => console.error(err));
        });
    }
});
