import * as React from "react";
import { Repo } from "./Repo";
import { useState, useEffect } from "react";

export function App(): JSX.Element {
  let [repos, setRepos] = useState([]);

  useEffect(() => {
      const getData = async (): Promise<void> => {
          const response = await fetch('/api/repos');
          repos = await response.json();
          setRepos(repos)
      };
      getData();
  }, []);

  if (repos.length === 0) {
    return (
      <div className="px-20 py-20">
        <p className="text-center subtle">No repos added yet</p>
      </div>
    );
  }

  return (
    <div className="px-20 py-20">
      <div className="grid">
        {repos.map(repo => (
          <Repo repo={repo} />
        ))}
      </div>{" "}
    </div>
  );
}
