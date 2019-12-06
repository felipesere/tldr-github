import React from "react";
import { data } from "./data";
import "./app.css";

const Master = ({ master }) => {
  return (
    <div>
      {master.commits} new commits on <code>master</code>
    </div>
  );
};

const GlowBox = ({ content }) => {
  return (
    <div className="glow-box">
      <a href={content.link}>{content.title}</a>
      <p className="is-size-7">by {content.by}</p>
    </div>
  );
};

const Issues = ({ issues }) => {
  return (
    <div>
      {issues.length} new Issues:
      {issues.map(issue => <GlowBox content={issue} />)}
    </div>
  );
};

const PullRequests = ({ prs }) => {
  return (
    <div>
      {prs.length} new Pull Requests:
      {prs.map(pr => <GlowBox content={pr} />)}
    </div>
  );
};

const Recent = ({ master }) => {
  return (
    <div>
      <h2 className="recent-activity">Recent activity</h2>
      <Master master={master} />
    </div>
  );
};

const Activity = ({ activity }) => {
  return (
    <>
      <Recent master={activity.master} />
      <div className="stack">
        <PullRequests prs={activity.prs} />
        <Issues issues={activity.issues} />
      </div>
    </>
  );
};

const LastCommit = ({ lastCommit }) => {
  return (
    <div>
      Last commit on <code>{lastCommit.branch}</code> {lastCommit.on} by{" "}
      {lastCommit.by}:
      <div className="commit">{lastCommit.comment}</div>
    </div>
  );
};

const Repo = props => {
  return (
    <article className="card">
      <header className="card-header">
        <div className="card-header-title">
          <p className="grow">{props.title}</p>
          <i className="icon ion-md-settings" />
        </div>
      </header>
      <div className="card-content grow">
        <div className="content stack">
          <LastCommit lastCommit={props.lastCommit} />
          <Activity activity={props.activity} />
        </div>
      </div>
      <footer className="card-footer">
        <p className="is-size-7 card-footer-item">Last update 2min ago</p>
      </footer>
    </article>
  );
};

const InputNewRepo = () => {
  return (
    <div className="field has-addons">
      <div className="control has-icons-right grow">
        <input className="input" type="text" placeholder="Add new repo" />
        <span className="icon is-small is-right">
          <i className="icon ion-md-checkmark" />
        </span>
      </div>
      <div className="control">
        <button className="button is-info">Add</button>
      </div>
    </div>
  );
};

const AddNewRepo = () => {
  return (
    <article className="card">
      <div className="card-content content grow">
        <InputNewRepo />
      </div>
    </article>
  );
};

const AllRepos = props => {
  return (
    <div className="grid">
      {props.repos.map(repo => <Repo {...repo} />)}
      <AddNewRepo />
    </div>
  );
};

const App = () => {
  return <AllRepos repos={data} />;
};

export default App;
