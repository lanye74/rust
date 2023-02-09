const input = document.getElementById("input") as HTMLInputElement;
input.addEventListener("keyup", () => search(input.value));

const output = document.getElementById("output") as HTMLPreElement;

const championsText = await fetch("./champs.txt").then(res => res.text());
const champions = championsText.split("\r\n");



// ignore capitalization, just for emphasis
// goal: given query "ww", I should be able to get the result "WarWick"
// given query "di", I should be able to get both kassaDIn and morDekaIser
// given query "nai", I should be able to get both NAmI and Nunu And wIllump
// given query "aa", I should be able to get both AAtrox and tAliyAh
// given query "aaa", I should be able to get both kAtArinA and mAlzAhAr
// given query "aka", I should be able to get both AKAli and AKshAn
// ^ this is the annoying one. it requires me to keep track of characters found and their last position
// wait. can't I use indexOf from a starting position...?

// getting closer. but not quite there
function search(query: string) {
	const queryCharacters = query.trim().toLowerCase().split("");

	// stored as [championIndex, index of last character matched]
	let filtered: Map<string, number> = new Map();


	for(const [searchIndex, character] of Object.entries(queryCharacters)) {
		for(const [championIndex, champion] of Object.entries(champions)) {
			let characterIndex = champion.toLowerCase().indexOf(character);

			if(characterIndex === -1) {
				filtered.delete(championIndex);
				continue;
			}

			if(!filtered.has(championIndex) && searchIndex === "0") {
				filtered.set(championIndex, characterIndex);
				continue;
			}

			if(!filtered.has(championIndex) && searchIndex !== "0") {
				continue;
			}

			// champion exists in filtered and contains this character
			characterIndex = champion.toLowerCase().indexOf(character, filtered.get(championIndex)! + 1);
			if(characterIndex === -1) {
				// character exists in the string, but it comes before the most recent found character
				filtered.delete(championIndex);
				continue;
			}

			// we're good!
			filtered.set(championIndex, characterIndex);

			console.log(filtered)
		}
	}

	// debugger

	let filteredChampions = Array.from(filtered.keys());

	const outputArray = filteredChampions.map(champIndex => champions[parseInt(champIndex)]);

	output.textContent = outputArray.join("\n");
}



// needed to make this a "true" module
export {};
