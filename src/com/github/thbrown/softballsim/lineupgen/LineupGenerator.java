package com.github.thbrown.softballsim.lineupgen;

import com.main.thbrown.softballsim.lineup.BattingLineup;

public interface LineupGenerator {
	
	/**
	 * After all games for a given lineup have been simulated, this method should get the next
	 * possible lineup. This method should return null when there are no more lineups to simulate.
	 */
	public BattingLineup getNextLienup();

	/**
	 * Pull data from all files in the immediate supplied directory. This is called before any calls
	 * to {@link #getNextLienup()}
	 */
	void readInDataFromFile(String statsPath);
	
}