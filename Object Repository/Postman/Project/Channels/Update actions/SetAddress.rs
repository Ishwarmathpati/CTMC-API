<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetAddress</name>
   <tag></tag>
   <elementGuidId>b98c3e48-8d4c-4f6c-992f-b2dda5ff4542</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${channel-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setAddress\&quot;,\n            \&quot;address\&quot; : {\n              \&quot;key\&quot; : \&quot;exampleKey\&quot;,\n              \&quot;title\&quot; : \&quot;My Address\&quot;,\n              \&quot;salutation\&quot; : \&quot;Mr.\&quot;,\n              \&quot;firstName\&quot; : \&quot;Example\&quot;,\n              \&quot;lastName\&quot; : \&quot;Person\&quot;,\n              \&quot;streetName\&quot; : \&quot;Example Street\&quot;,\n              \&quot;streetNumber\&quot; : \&quot;4711\&quot;,\n              \&quot;additionalStreetInfo\&quot; : \&quot;Backhouse\&quot;,\n              \&quot;postalCode\&quot; : \&quot;80933\&quot;,\n              \&quot;city\&quot; : \&quot;Exemplary City\&quot;,\n              \&quot;region\&quot; : \&quot;Exemplary Region\&quot;,\n              \&quot;state\&quot; : \&quot;Exemplary State\&quot;,\n              \&quot;country\&quot; : \&quot;DE\&quot;,\n              \&quot;company\&quot; : \&quot;My Company Name\&quot;,\n              \&quot;department\&quot; : \&quot;Sales\&quot;,\n              \&quot;building\&quot; : \&quot;Hightower 1\&quot;,\n              \&quot;apartment\&quot; : \&quot;247\&quot;,\n              \&quot;pOBox\&quot; : \&quot;2471\&quot;,\n              \&quot;phone\&quot; : \&quot;+49 89 12345678\&quot;,\n              \&quot;mobile\&quot; : \&quot;+49 171 2345678\&quot;,\n              \&quot;email\&quot; : \&quot;email@example.com\&quot;,\n              \&quot;fax\&quot; : \&quot;+49 89 12345679\&quot;,\n              \&quot;additionalAddressInfo\&quot; : \&quot;no additional Info\&quot;,\n              \&quot;externalId\&quot; : \&quot;Information not needed\&quot;\n            }\n          }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>7cdfb70d-d7f7-4e3e-b48f-8ad5bd61f99f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a96ce511-e6d7-48e2-a543-a4cd406d840a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/channels/${channel-id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>0546da84-7ecc-409e-b821-b1904aa175b0</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>d2d818d2-b163-4d47-8c99-c3f53af2b616</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.channel-id</defaultValue>
      <description></description>
      <id>cffe9419-158a-4caa-9cf0-3085d2c0306e</id>
      <masked>false</masked>
      <name>channel-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.channel-version</defaultValue>
      <description></description>
      <id>5547bed5-3f27-4d84-a592-0cb22f3aaed8</id>
      <masked>false</masked>
      <name>channel-version</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
