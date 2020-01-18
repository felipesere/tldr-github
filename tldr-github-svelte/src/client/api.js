import {newError} from '../errors/errorStore.js';

export const addRepo = async (name) => {
    try {
        return await doPost("/repos", {name});
    } catch (e) {
        newError(`Could not add repo ${name}`);
        return Promise.reject(e)
    }
};

export const deleteRepo = async (repoId) => {
    try {
        return await doDelete(`/repos/${repoId}`)
    } catch (e) {
        newError(`Unable to delete repo: ${e}`)
    }
};

const doPost = async (path, data) => {
    return fetch(`/api${path}`, {
        "body": JSON.stringify(data),
        "method": "POST",
        "headers": {
            "Content-Type": "application/json",
        },
    }).then(processRequest)
};

const doDelete = (path) => {
    return fetch(`/api${path}`, {"method": "DELETE"}).then(processRequest)
};

const processRequest = async (response) => {
    if (!response.ok) {
        return Promise.reject({error: response.body})
    } else {
        try {
            return await response.json()
        } catch (e) {
            return Promise.resolve({})
        }
    }
};