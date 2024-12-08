class Group {
    async getAll() {
        let groups = await fetch("/get_groups");
        return await groups.json();
    }
}

export default Group;