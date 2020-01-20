import {toRequest} from "../extraExpectations";
import {enableMocks} from 'jest-fetch-mock'
import {addRepo, trackItems} from "../../src/client/api";
import {error} from '../../src/errors/errorStore.js'

enableMocks();

expect.extend({toRequest});

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
        fetchMock.mockReject("it failed");
        let err;
        error.subscribe((e) => err = e);

        await expect(addRepo('my-repo')).rejects.toEqual("it failed");
        expect(fetchMock).toRequest('POST', '/api/repos', {name: 'my-repo'});
        expect(err).toEqual({msg: 'Could not add repo my-repo'})
    });

    it('can add new items to track', async () => {
        fetchMock.mockResponse();
        await trackItems(123, [{kind: 'pr', nr: 1}, {kind: 'issue', nr: 7}]);

        expect(fetchMock).toRequest('POST', '/api/repos/123/tracked', [{kind: 'pr', nr: 1}, {kind: 'issue', nr: 7}])
    })
});