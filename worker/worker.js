addEventListener('fetch', event => {
    event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
    const { get_response } = wasm_bindgen;
    await wasm_bindgen(wasm)

    let body;
    if (request.body) {
        body = await request.text();
    } else {
        body = "";
    }

    let headers = {};
    for (let key of request.headers.keys()) {
        headers[key] = request.headers.get(key);
    }

    const response = await get_response({
        method: request.method,
        headers: headers,
        path: request.url,
        body: body
    });

    return new Response(response.body, {
        status: response.status,
        headers: response.headers
    });
}
