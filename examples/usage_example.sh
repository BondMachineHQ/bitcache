#!/bin/bash
# Example usage script for bitcache

# This script demonstrates how to use bitcache in a typical workflow

# Configuration
REPO="https://github.com/BondMachineHQ/bitcachedata.git"
SOURCE_FILE="example.vhd"
BITSTREAM_FILE="example.bit"
TARGET_PATH="builds/examples"

echo "================================"
echo "bitcache Example Workflow"
echo "================================"
echo ""

# Step 1: Create example files (simulating a real workflow)
echo "Step 1: Creating example source file..."
cat > $SOURCE_FILE << 'EOF'
-- Example VHDL file
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

entity example is
    Port ( clk : in STD_LOGIC;
           led : out STD_LOGIC);
end example;

architecture Behavioral of example is
begin
    process(clk)
    begin
        if rising_edge(clk) then
            led <= '1';
        end if;
    end process;
end Behavioral;
EOF

# Create a dummy bitstream file
echo "Creating example bitstream file..."
dd if=/dev/urandom of=$BITSTREAM_FILE bs=1024 count=10 2>/dev/null

# Step 2: Compute MD5 for reference
echo ""
echo "Step 2: Computing MD5 of source file..."
MD5=$(md5sum $SOURCE_FILE | awk '{print $1}')
echo "Source MD5: $MD5"

# Step 3: Publish the bitstream
echo ""
echo "Step 3: Publishing bitstream to repository..."
echo "Command: bitcache publish --repo $REPO --source $SOURCE_FILE --bitstream $BITSTREAM_FILE --path $TARGET_PATH"
echo ""
echo "NOTE: Update REPO variable with your actual git repository URL before running this command"
# Uncomment the line below when you have a real repository
#bitcache publish --repo "$REPO" --source "$SOURCE_FILE" --bitstream "$BITSTREAM_FILE" --path "$TARGET_PATH"

# Step 4: Retrieve the bitstream (in a real scenario, you'd do this later or on another machine)
echo ""
echo "Step 4: To retrieve the bitstream later, use:"
echo "bitcache get --repo $REPO --md5 $MD5"
echo ""
echo "This will copy the bitstream to your current directory"
#bitcache get --repo $REPO --md5 $MD5

# Cleanup
echo ""
echo "Cleaning up example files..."
rm -f $SOURCE_FILE $BITSTREAM_FILE

echo ""
echo "================================"
echo "Example complete!"
echo "================================"
echo ""
echo "To use bitcache with your own repository:"
echo "1. Update the REPO variable in this script"
echo "2. Replace example files with your actual source and bitstream"
echo "3. Uncomment the bitcache publish command"
echo "4. Run this script"
