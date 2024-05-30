# Inspector
## Overview

Inspector is a poker tracking application designed to enhance your experience on PokerStars by providing real-time statistics about your opponents, similar to popular tools like PokerTracker. This project leverages modern web technologies to deliver a smooth, efficient, and powerful user interface and backend.

## Project Objective
The primary goal of Inspector is to offer a heads-up display (HUD) that provides crucial statistics about players during gameplay. By analyzing past hands and tracking real-time progress, the tool aims to give players an edge by presenting valuable insights and data about their opponents' behaviors and tendencies.

## Technologies Used
### Frontend
- Svelte: A modern JavaScript framework for building user interfaces. Svelte's reactive nature and efficient compilation make it an excellent choice for developing a dynamic and responsive HUD.
- Backend
- Rust: Known for its performance and safety, Rust is used to ensure the backend is both fast and reliable. The choice of Rust aligns with the need for real-time data processing and high efficiency.
- Tauri: Tauri is a toolkit for building desktop applications using web technologies. It bridges the frontend and backend, allowing the use of Svelte for the UI while leveraging Rust's capabilities for backend processing.
- Diesel: An ORM (Object-Relational Mapping) library for Rust, Diesel is used to manage the SQLite database interactions, providing a robust and type-safe way to handle database queries.
- SQLite: A lightweight, disk-based database, SQLite is used for storing hand histories and player statistics, making it easy to query and analyze past game data.
## Project Progress
### Completed
- Frontend Interface: A basic frontend has been developed using Svelte, providing the structure for the HUD and other UI components. Although it currently shows no data, the foundation is set for further enhancements.
- Backend Infrastructure: The backend is functional, capable of handling core operations such as database interactions and basic data processing. Diesel and SQLite integration is complete, allowing for the storage and retrieval of game data.
### In Progress
- Real-Time Tracking: The backend requires further development to track game progress in real-time. This involves implementing mechanisms to capture and process hand data as games are played.
- HUD Implementation: The heads-up display, which is a crucial feature of the project, is under development. The goal is to overlay real-time statistics and insights on the PokerStars interface, giving players immediate access to valuable information about their opponents.

## Future Plans
- Enhanced Data Analytics: Improve the analytics capabilities to provide more detailed and actionable insights.
- User Interface Improvements: Refine the Svelte frontend to display real-time data effectively and intuitively.
- Optimization and Performance Tuning: Ensure that the application runs smoothly without impacting the performance of the PokerStars client.
