package com.github.thbrown.softballsim.lineup;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

import com.github.thbrown.softballsim.Player;

/**
 * Batting order that strictly alternates between two groups of players. (i.e.
 * women and men or older and younger). Batters in a group with less players
 * will bat more often.
 * 
 * TODO: add optional out for skipping batter
 *
 * @author thbrown
 */
public class AlternatingBattingLineup implements BattingLineup {

  private List<Player> groupA;
  private List<Player> groupB;

  private int groupAHitterIndex = 0;
  private int groupBHitterIndex = 0;

  private boolean selectFromGroupA = true;

  public AlternatingBattingLineup(List<Player> groupA, List<Player> groupB) {
    this.groupA = groupA;
    this.groupB = groupB;
    if (groupA.size() <= 0 || groupB.size() <= 0) {
      throw new IllegalArgumentException(String.format(
          "You must include at least one player of each gender.\n" +
              "Males: %s Females: %s .", groupA.size(), groupB.size()));
    }
  }

  @Override
  public Player getNextBatter() {
    Player selection;
    if (selectFromGroupA) {
      selection = groupA.get(groupAHitterIndex);
      groupAHitterIndex = (groupAHitterIndex + 1) % groupA.size();
    } else {
      selection = groupB.get(groupBHitterIndex);
      groupBHitterIndex = (groupBHitterIndex + 1) % groupB.size();
    }
    selectFromGroupA = !selectFromGroupA;
    return selection;
  }

  @Override
  public void reset() {
    groupAHitterIndex = 0;
    groupBHitterIndex = 0;
  }

  @Override
  public String toString() {
    StringBuilder result = new StringBuilder();
    result.append("GroupA").append("\n");
    for (Player p : groupA) {
      result.append("\t").append(p).append("\n");
    }

    result.append("GroupB").append("\n");
    for (Player p : groupB) {
      result.append("\t").append(p).append("\n");
    }
    return result.toString();
  }
  
  @Override
  public Map<String, List<String>> toMap() {
    Map<String,List<String>>result = new HashMap<>();
    result.put("GroupA", groupA.stream().map(p -> p.getName().trim()).collect(Collectors.toList()));
    result.put("GroupB", groupB.stream().map(p -> p.getName().trim()).collect(Collectors.toList()));
    return result;
  }
  
  @Override
  public BattingLineup getRandomSwap() {
    throw new UnsupportedOperationException();
  }
  
}
