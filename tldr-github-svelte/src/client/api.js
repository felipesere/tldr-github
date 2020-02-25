import {newError} from '../errors/errorStore.js';

const to_url = (name) => name.replace("/", "---");

export const proxy = async (name) => {
    try {
        return await fetch(`/api/repos/${to_url(name)}/proxy`);
    } catch (e) {
        newError(`Could not proxy to repo ${name}`);
        return Promise.reject(e)
    }
};

export const addRepo = async (name) => {
    try {
        return await doPost("/repos", {name});
    } catch (e) {
        newError(`Could not add repo ${name}`);
        return Promise.reject(e)
    }
};

export const deleteRepo = async (name) => {
    try {
        return await doDelete(`/repos/${name}`)
    } catch (e) {
        newError(`Unable to delete repo: ${e}`)
    }
};

export const trackItems = async (name, items) => {
    try {
        return await doPost(`/repos/${to_url(name)}/tracked`, items)
    } catch (e) {
        newError(`Unable to add items to repo ${repoId}: ${e}`)
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
