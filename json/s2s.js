const fs = require('fs');
const path = require('path');

// Paths to the input and output files
const inputFilePath = path.join(__dirname, 'songs.json');
const outputFilePath = path.join(__dirname, 'sets.json');

// Function to extract beatmapset IDs
const extractBeatmapsetIds = (data) => {
  const beatmapsetIds = data.map(item => item.song_id.toString());
  return { beatmapset_ids: beatmapsetIds };
};

// Read the input file
fs.readFile(inputFilePath, 'utf8', (err, data) => {
  if (err) {
    console.error('Error reading songs.json:', err);
    return;
  }

  // Parse the input data
  const parsedData = JSON.parse(data);

  // Extract the beatmapset IDs
  const result = extractBeatmapsetIds(parsedData);

  // Write the output file
  fs.writeFile(outputFilePath, JSON.stringify(result, null, 2), 'utf8', (err) => {
    if (err) {
      console.error('Error writing sets.json:', err);
      return;
    }

    console.log('sets.json has been saved.');
  });
});