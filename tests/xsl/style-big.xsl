<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'
		version="3.0">
  <xsl:output indent="yes"/>
  <xsl:strip-space elements="*"/>

  <xsl:param name="howbig" select="1"/>

  <xsl:template match='child::Test'>
    <big>
      <xsl:for-each select="1 to $howbig">
        <element>
          <xsl:apply-templates select="."/>
        </element>
      </xsl:for-each>
    </big>
  </xsl:template>
  <xsl:template match='child::text()'>
    <xsl:sequence select="."/>
  </xsl:template>
</xsl:stylesheet>
