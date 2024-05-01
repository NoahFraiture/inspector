<nav>
	<a href="/">Home</a>
	<a href="/graph">Graph</a>
	<a href="/profile">Profile</a>
</nav>

<main>
	<slot></slot>
</main>

<style>
	nav {
		width: 200px; /* Set the width of the sidebar */
		height: 100vh; /* Full height */
		position: fixed; /* Fixed Sidebar (stay in place on scroll) */
		z-index: 1; /* Stay on top */
		top: 0; /* Stay at the top */
		left: 0; /* Stay on the left */
		background-color: #111; /* Black background color */
		overflow-x: hidden; /* Disable horizontal scroll */
		padding-top: 20px; /* Some padding at the top */
	}

	nav a {
		padding: 10px 15px; /* Padding inside the links */
		text-decoration: none; /* No underline on links */
		font-size: 18px; /* Increase font size */
		color: white; /* White text color */
		display: block; /* Make the links appear below each other */
	}

	nav a:hover {
		background-color: #575757; /* Add a hover effect */
	}

	main {
		margin-left: 200px; /* Add a left margin to the main content to prevent overlay */
		padding: 20px; /* Padding around the main content */
	}
</style>
