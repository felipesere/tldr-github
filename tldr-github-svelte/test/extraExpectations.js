const fail = (m) => ({pass: false, message: () => m});
const pass = () => ({pass: true});

export const toRequest = (receiver, expectedMethod, expectedPath, expectedBody) => {
    if (receiver.mock.calls.length === 0) {
        return fail("no calls to fetch API");
    }
    let [path, {body, method}] = receiver.mock.calls[0];

    if (path !== expectedPath) {
        return fail(`Unexpected path ${path}, wanted: ${expectedPath}`)
    }

    if (method !== expectedMethod) {
        return fail(`Unexpected method ${method}, wanted: ${expectedMethod}`)
    }

    if (JSON.stringify(expectedBody) !== body) {
        return fail(`Unexpected body '${body}', wanted: '${JSON.stringify(expectedBody)}'`)
    }

    return pass()
};
