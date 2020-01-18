import {enableMocks} from 'jest-fetch-mock'
import {addRepo} from "../../src/client/api";
import {error} from '../../src/errors/errorStore.js'

enableMocks();

expect.extend({
    toRequest(receiver, expectedMethod, expectedPath, expectedBody) {
        if (receiver.mock.calls.length === 0) {
            return {
                pass: false,
                message: () => "No calls to mock",
            }
        }
        let [path, {body, method}] = receiver.mock.calls[0];

        if (path !== path) {
            return {pass: false, message: () => `Unexpected path ${path}, wanted: ${expectedPath}`}
        }

        if (method !== expectedMethod) {
            return {pass: false, message: () => `Unexpected method ${method}, wanted: ${expectedMethod}`}
        }

        if (JSON.stringify(expectedBody) !== body) {
            return {pass: false, message: () => `Unexpected body '${body}', wanted: '${JSON.stringify(expectedBody)}'`}
        }

        return {
            pass: true,
        }
    },
});

describe('the client API', () => {

    beforeEach(() => {
        fetchMock.resetMocks();
    });

    it('can add new repos', async () => {
        fetchMock.mockResponse({});
        await addRepo('my-repo');

        expect(fetchMock).toRequest('POST', '/api/repos', {name: 'my-repo'})
    });

    it('reports errors when it fails', async () => {
        fetchMock.mockReject("it failed")
        let err;
        error.subscribe((e) => err = e);

        await expect(addRepo('my-repo')).rejects.toEqual("it failed");
        expect(fetchMock).toRequest('POST', '/api/repos', {name: 'my-repo'})
        expect(err).toEqual({msg: 'Could not add repo my-repo'})
    })
});