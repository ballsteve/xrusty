<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Test'><HTML><xsl:apply-templates/></HTML></xsl:template>
  <xsl:template match='child::Level1'><DIV><xsl:apply-templates/></DIV></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select="."/></xsl:template>
</xsl:stylesheet>
