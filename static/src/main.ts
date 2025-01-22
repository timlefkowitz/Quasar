const form = document.getElementById('postForm') as HTMLFormElement;
const responseDiv = document.getElementById('response') as HTMLDivElement;

form.addEventListener('submit', async (e) => {
    e.preventDefault();
    const postContent = (document.getElementById('postContent') as HTMLTextAreaElement).value;

    try {
        const response = await fetch('http://127.0.0.1:8080/post', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ content: postContent }),
        });

        const result = await response.json();
        responseDiv.innerText = `Server response: ${JSON.stringify(result)}`;
    } catch (error) {
        responseDiv.innerText = `Error: ${error}`;
    }
});
