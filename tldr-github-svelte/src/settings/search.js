export const search = (items, search, fields) => {
    return items.filter(item => {
        for (const field of fields) {
            const value = item[field];

            if (!value) {
                continue
            }

            if (Array.isArray(value)) {
                if (value.some((v) => search.test(v))) {
                    return true
                }
            }

            if (search.test(value)) {
                return true
            }
        }
        return false
    })
};
