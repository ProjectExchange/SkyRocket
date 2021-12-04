<h1 align="center">
  <br />
  <img src="https://raw.githubusercontent.com/ProjectExchange/SkyRocket/main/src/assets/logo.png" alt="SkyRocket" width="150"></a>
  <br /><br />
  SkyRocket
  <br />
</h1>

<h3 align="center">🤖 Automated SAST Pipeline</h3>
<p align="center">
  <img src="https://github.com/ProjectExchange/SkyRocket/actions/workflows/sonarqube.yml/badge.svg" alt="SonarQube CI" />
</p>

<h3 align="center">🔒 SonarQube Monitored</h3>
<p align="center">
  <img src="https://sonarqube.projectexchange.org/api/project_badges/measure?project=projectexchange%3Askyrocket&metric=alert_status&token=5db82bec94ca4d079b39f1021383beb1b3723d98" alt="Quality Gate Status" />
  <img src="https://sonarqube.projectexchange.org/api/project_badges/measure?project=projectexchange%3Askyrocket&metric=sqale_rating&token=5db82bec94ca4d079b39f1021383beb1b3723d98" alt="Maintainability Rating" />
  <img src="https://sonarqube.projectexchange.org/api/project_badges/measure?project=projectexchange%3Askyrocket&metric=reliability_rating&token=5db82bec94ca4d079b39f1021383beb1b3723d98" alt="Reliability Rating" />
  <img src="https://sonarqube.projectexchange.org/api/project_badges/measure?project=projectexchange%3Askyrocket&metric=security_rating&token=5db82bec94ca4d079b39f1021383beb1b3723d98" alt="Security Rating" />
</p>
<p align="center">
  <img src="https://sonarqube.projectexchange.org/api/project_badges/measure?project=projectexchange%3Askyrocket&metric=ncloc&token=5db82bec94ca4d079b39f1021383beb1b3723d98" alt="Lines of Code" />
  <img src="https://sonarqube.projectexchange.org/api/project_badges/measure?project=projectexchange%3Askyrocket&metric=vulnerabilities&token=5db82bec94ca4d079b39f1021383beb1b3723d98" alt="Vulnerabilities" />
  <img src="https://sonarqube.projectexchange.org/api/project_badges/measure?project=projectexchange%3Askyrocket&metric=sqale_index&token=5db82bec94ca4d079b39f1021383beb1b3723d98" alt="Technical Debt" />
</p>
<p align="center">
  <img src="https://sonarqube.projectexchange.org/api/project_badges/measure?project=projectexchange%3Askyrocket&metric=code_smells&token=5db82bec94ca4d079b39f1021383beb1b3723d98" alt="Code Smells" />
  <img src="https://sonarqube.projectexchange.org/api/project_badges/measure?project=projectexchange%3Askyrocket&metric=coverage&token=5db82bec94ca4d079b39f1021383beb1b3723d98" alt="Coverage" />
  <img src="https://sonarqube.projectexchange.org/api/project_badges/measure?project=projectexchange%3Askyrocket&metric=duplicated_lines_density&token=5db82bec94ca4d079b39f1021383beb1b3723d98" alt="Duplicated Lines (%)" />
  <img src="https://sonarqube.projectexchange.org/api/project_badges/measure?project=projectexchange%3Askyrocket&metric=bugs&token=5db82bec94ca4d079b39f1021383beb1b3723d98" alt="Bugs" />
</p>

---

## What is SkyRocket?

SkyRocket is a secure booking service for flights, written in Rust and TypeScript.

## Folder overview

| Folder     | Description                                                                                                        |
| ---------- | ------------------------------------------------------------------------------------------------------------------ |
| `.github`  | All files used by GitHub. For example, for the workflows to automatically run the SAST pipeline.                   |
| `.vscode`  | The configuration file for the used IDE.                                                                           |
| `backend`  | Contains all files for the SkyRocket backend. It is written in Rust and uses dependencies like Rocket.             |
| `docker`   | Files for the development setup for spawning a clean MariaDB and Redis database in a Docker container.             |
| `docs`     | This folder contains the SkyRocket documentation as well as the architectural design.                              |
| `frontend` | Contains all files for the Skyrocket frontend. This is written in TypeScript using Angular.                        |
| `lib`      | This folder is used for the necessary depedency packages generated for the frontend for connection to the backend. |
| `scripts`  | Scripts for automated code generation. Running in some new spawned Docker container.                               |
